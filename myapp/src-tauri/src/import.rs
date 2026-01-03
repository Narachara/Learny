use tauri_plugin_dialog::{ FileDialogBuilder, FilePath, DialogExt };
use crate::db::{ open_db, add_deck, add_card , save_card_blocks, restore_card_metadata };
use futures::channel::oneshot;
use tauri::Manager;
use crate::export::DeckExport;
use tauri_plugin_bliet::BlietExt;
use std::collections::HashMap;
use std::io::{Cursor, Read};
use uuid::Uuid;
use zip::ZipArchive;

/* 
TODO:

🔹 Transactions

Wrap import in a DB transaction so partial imports roll back on failure.

🔹 Progress feedback

Emit progress events for large decks.

🔹 Integrity checks

Add a checksum to export.json.

🔹 Backward compatibility

Support export_version = 1 JSON-only imports.

🔹 Streaming ZIP

For very large decks, stream extraction instead of buffering. */


pub fn import_deck_zip(
    app: &tauri::AppHandle,
    zip_bytes: &[u8],
) -> Result<DeckExport, String> {
    let reader = Cursor::new(zip_bytes);
    let mut zip = ZipArchive::new(reader).map_err(|e| e.to_string())?;

    // 1️⃣ Read export.json
    let mut json = String::new();
    zip.by_name("export.json")
        .map_err(|_| "export.json not found in archive".to_string())?
        .read_to_string(&mut json)
        .map_err(|e| e.to_string())?;

    let mut export: DeckExport = serde_json::from_str(&json)
        .map_err(|e| e.to_string())?;

    if export.export_version != 2 {
        return Err(format!(
            "Unsupported export version: {}",
            export.export_version
        ));
    }

    // 2️⃣ Prepare app files dir
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| e.to_string())?;
    let files_dir = app_data_dir.join("files");
    std::fs::create_dir_all(&files_dir)
        .map_err(|e| e.to_string())?;

    // 3️⃣ Extract files + build path map
    let mut path_map: HashMap<String, String> = HashMap::new();

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).map_err(|e| e.to_string())?;
        let zip_path = file.name().to_string();

        if !zip_path.starts_with("files/") {
            continue;
        }

        let ext = std::path::Path::new(&zip_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("bin");

        let new_name = format!("{}.{}", Uuid::new_v4(), ext);
        let dest_path = files_dir.join(&new_name);

        let mut out = std::fs::File::create(&dest_path)
            .map_err(|e| e.to_string())?;

        std::io::copy(&mut file, &mut out)
            .map_err(|e| e.to_string())?;

        path_map.insert(
            zip_path,
            format!("files/{}", new_name),
        );
    }

    // 4️⃣ Rewrite paths inside blocks
    for card in export.cards.iter_mut() {
        for block in card.all_blocks_mut() {
            if let Some(path) = block.file_path_mut() {
                if let Some(new_path) = path_map.get(path) {
                    *path = new_path.clone();
                }
            }
        }
    }

    Ok(export)
}




pub fn import_deck_export(
    app: &tauri::AppHandle,
    export: DeckExport,
) -> Result<i64, String> {
    // ⚠️ IMPORTANT:
    // Do NOT reuse export.deck.id or card.id

    // 1️⃣ Create deck
    let new_deck_id = add_deck(
        app.clone(),
        export.deck.name.clone(),
    )?;

    // 2️⃣ Create cards
    for card in export.cards {
        let new_card_id = add_card(
            app.clone(),
            new_deck_id,
            card.name.clone(),
        )?;

        save_card_blocks(
            app.clone(),
            new_card_id,
            card.front_blocks,
            card.back_blocks,
        )?;

        restore_card_metadata(
            app,
            new_card_id,
            card.created_at,
            card.times_seen,
            card.times_correct,
            card.tags,
        )?;
    }


    Ok(new_deck_id)
}


#[tauri::command]
pub async fn import_deck(app: tauri::AppHandle) -> Result<i64, String> {
    let Some(bytes) = app
        .bliet() // calling a plugin function via appstate 
        .pick_import_file() // this function is from plugin
        .await
        .map_err(|e| e.to_string())?
    else {
        return Ok(0);
    };

    let export = import_deck_zip(&app, &bytes)?;
    import_deck_export(&app, export)
}
