use crate::db::{get_card, get_deck, open_db};
use futures::channel::oneshot;
use serde::{Deserialize, Serialize};
use shared::models::{derive_export_path, Block, Card, Deck};
use std::io::Write;
use tauri::Manager;
use tauri_plugin_dialog::{DialogExt, FileDialogBuilder, FilePath};
use zip::{write::FileOptions, ZipWriter};



// =======================
// Models
// =======================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckExport {
    pub export_version: u32,
    pub deck: Deck,
    pub cards: Vec<Card>,
}

#[derive(Debug)]
pub struct ExportFile {
    pub src_virtual: String, // e.g. "files/uuid.png"
    pub zip_path: String,    // e.g. "files/card_0/front_0.png"
}



// =======================
// DB helpers
// =======================

pub fn get_card_ids(app: &tauri::AppHandle, deck_id: i64) -> Result<Vec<i64>, String> {
    let conn = open_db(app)?;

    let mut stmt = conn
        .prepare("SELECT id FROM card WHERE deck_id = ? ORDER BY created_at DESC")
        .map_err(|e| e.to_string())?;

    let rows = stmt
        .query_map([deck_id], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    let mut ids = Vec::new();
    for id in rows {
        ids.push(id.map_err(|e| e.to_string())?);
    }

    Ok(ids)
}

pub fn export_deck_cards(app: &tauri::AppHandle, deck_id: i64) -> Result<Vec<Card>, String> {
    let card_ids = get_card_ids(app, deck_id)?;

    let mut cards = Vec::new();
    for id in card_ids {
        cards.push(get_card(app.clone(), id)?);
    }

    Ok(cards)
}



// =======================
// Export helpers
// =======================

pub fn collect_export_files(cards: &[Card]) -> Vec<ExportFile> {
    let mut files = Vec::new();

    for (card_index, card) in cards.iter().enumerate() {
        for (block_index, block) in card.front_blocks.iter().enumerate() {
            if let Some(src) = block.file_path() {
                files.push(ExportFile {
                    src_virtual: src.to_string(),
                    zip_path: derive_export_path(card_index, block_index, "front", src),
                });
            }
        }

        for (block_index, block) in card.back_blocks.iter().enumerate() {
            if let Some(src) = block.file_path() {
                files.push(ExportFile {
                    src_virtual: src.to_string(),
                    zip_path: derive_export_path(card_index, block_index, "back", src),
                });
            }
        }
    }

    files
}

pub fn resolve_virtual_path(
    app: &tauri::AppHandle,
    virtual_path: &str,
) -> Result<std::path::PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    Ok(app_data_dir.join(virtual_path))
}


use std::collections::HashMap;

pub fn build_export_path_map(
    export_files: &[ExportFile],
) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for file in export_files {
        map.insert(
            file.src_virtual.clone(),
            file.zip_path.clone(),
        );
    }

    map
}



pub fn rewrite_cards_for_export(
    cards: &[Card],
    path_map: &std::collections::HashMap<String, String>,
) -> Vec<Card> {
    let mut rewritten = cards.to_vec(); // clone

    for card in rewritten.iter_mut() {
        for block in card.all_blocks_mut() {
            if let Some(path) = block.file_path_mut() {
                if let Some(new_path) = path_map.get(path) {
                    *path = new_path.clone();
                }
            }
        }
    }

    rewritten
}


// =======================
// ZIP builder
// =======================

pub fn build_deck_zip(
    app: &tauri::AppHandle,
    deck_id: i64,
) -> Result<Vec<u8>, String> {
    let deck = get_deck(app.clone(), deck_id)?;
    let cards = export_deck_cards(app, deck_id)?;

    // 1️⃣ Collect files
    let export_files = collect_export_files(&cards);

    // 2️⃣ Build path map
    let path_map = build_export_path_map(&export_files);

    // 3️⃣ Rewrite cards for export.json
    let exported_cards = rewrite_cards_for_export(&cards, &path_map);

    // 4️⃣ Build export model
    let export = DeckExport {
        export_version: 2,
        deck,
        cards: exported_cards,
    };


    let mut buffer = Vec::new();
    let cursor = std::io::Cursor::new(&mut buffer);
    let mut zip = ZipWriter::new(cursor);
    let options: FileOptions<()> = FileOptions::default();

    // 1️⃣ export.json
    let json = serde_json::to_string_pretty(&export)
        .map_err(|e| e.to_string())?;

    zip.start_file("export.json", options)
        .map_err(|e| e.to_string())?;
    zip.write_all(json.as_bytes())
        .map_err(|e| e.to_string())?;

    // 2️⃣ files
    for file in export_files {
        let src_path = resolve_virtual_path(app, &file.src_virtual)?;
        let data = std::fs::read(&src_path)
            .map_err(|e| format!("Failed to read {:?}: {}", src_path, e))?;

        zip.start_file(&file.zip_path, options)
            .map_err(|e| e.to_string())?;
        zip.write_all(&data)
            .map_err(|e| e.to_string())?;
    }

    zip.finish().map_err(|e| e.to_string())?;
    Ok(buffer)
}



// =======================
// Tauri command
// =======================

#[tauri::command]
pub async fn export_deck(
    app: tauri::AppHandle,
    deck_id: i64,
) -> Result<(), String> {
    let zip_bytes = tauri::async_runtime::spawn_blocking({
        let app = app.clone();
        move || build_deck_zip(&app, deck_id)
    })
    .await
    .map_err(|e| e.to_string())??;

    let (tx, rx) = oneshot::channel();

    FileDialogBuilder::new(app.dialog().clone())
        .set_file_name("deck-export.zip")
        .save_file(move |file| {
            let _ = tx.send(file);
        });

    let dest = match rx.await {
        Ok(Some(FilePath::Path(path))) => path,
        Ok(_) => return Ok(()), // user cancelled or non-path
        Err(_) => return Ok(()), // dialog canceled
    };

    tauri::async_runtime::spawn_blocking(move || {
        std::fs::write(dest, zip_bytes).map_err(|e| e.to_string())
    })
    .await.map_err(|e| e.to_string())??;

    Ok(())
}