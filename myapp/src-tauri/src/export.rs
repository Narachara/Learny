use crate::db::{ get_card, open_db, get_deck };
use tauri::Manager;
use serde::{Serialize, Deserialize};
use shared::models::{Card, Deck};
use tauri_plugin_dialog::{ FileDialogBuilder, FilePath, DialogExt };
use futures::channel::oneshot;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckExport {
    pub export_version: u32,
    pub deck: Deck,
    pub cards: Vec<Card>,
}


pub fn get_card_ids(app: &tauri::AppHandle, deck_id: i64) -> Result<Vec<i64>, String> {
    let conn = open_db(app)?;

    let mut stmt = conn.prepare(
        "SELECT id FROM card WHERE deck_id = ? ORDER BY created_at DESC"
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map([deck_id], |row| row.get(0))
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
        cards.push(get_card(app.clone(), id)?); // get_card sollte dann auch &AppHandle nehmen
    }

    Ok(cards)
}

pub fn export_deck_json(app: &tauri::AppHandle, deck_id: i64) -> Result<String, String> {
    let deck = get_deck(app.clone(), deck_id)?;
    let cards = export_deck_cards(app, deck_id)?;

    let export = DeckExport {
        export_version: 1,
        deck,
        cards,
    };

    serde_json::to_string_pretty(&export).map_err(|e| e.to_string())
}


#[tauri::command]
pub async fn export_deck(
    app: tauri::AppHandle,
    deck_id: i64,
) -> Result<(), String> {
    // 1️⃣ Generate export JSON (blocking → spawn_blocking)
    let json = tauri::async_runtime::spawn_blocking({
        let app = app.clone();
        move || export_deck_json(&app, deck_id)
    })
    .await
    .map_err(|e| e.to_string())??;

    // 2️⃣ Ask user where to save it (main thread)
    let (tx, rx) = oneshot::channel();

    FileDialogBuilder::new(app.dialog().clone())
        .set_file_name("deck-export.json")
        .save_file(move |file| {
            let _ = tx.send(file);
        });

    let dest = rx.await.map_err(|e| e.to_string())?;

    let Some(FilePath::Path(dest_path)) = dest else {
        // user cancelled → not an error
        return Ok(());
    };

    // 3️⃣ Write file (blocking → spawn_blocking)
    tauri::async_runtime::spawn_blocking(move || {
        std::fs::write(&dest_path, json)
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|e| e.to_string())??;

    Ok(())
}
