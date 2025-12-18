use tauri_plugin_dialog::{ FileDialogBuilder, FilePath, DialogExt };
use crate::db::{ open_db, add_deck, add_card , save_card_blocks };
use futures::channel::oneshot;
use tauri::Manager;
use crate::export::DeckExport;



pub fn import_deck_json(json: &str) -> Result<DeckExport, String> {
    let export: DeckExport =
        serde_json::from_str(json).map_err(|e| e.to_string())?;

    if export.export_version != 1 {
        return Err(format!(
            "Unsupported export version: {}",
            export.export_version
        ));
    }

    Ok(export)
}


#[tauri::command]
pub async fn import_deck(app: tauri::AppHandle) -> Result<i64, String> {
    // 1️⃣ Ask user for file
    let (tx, rx) = oneshot::channel();

    FileDialogBuilder::new(app.dialog().clone())
        .pick_file(move |file| {
            let _ = tx.send(file);
        });

    let Some(FilePath::Path(path)) = rx.await.map_err(|e| e.to_string())?
    else {
        return Ok(0); // user cancelled
    };

    // 2️⃣ Read + import on blocking thread
    let app_clone = app.clone();

    tauri::async_runtime::spawn_blocking(move || {
        let json = std::fs::read_to_string(path)
            .map_err(|e| e.to_string())?;

        let export = import_deck_json(&json)?;
        import_deck_export(&app_clone, export)
    })
    .await
    .map_err(|e| e.to_string())?
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
        // Create empty card
        let new_card_id = add_card(
            app.clone(),
            new_deck_id,
            card.name.clone(),
        )?;

        // Fill card blocks
        save_card_blocks(
            app.clone(),
            new_card_id,
            card.front_blocks,
            card.back_blocks,
        )?;
    }

    Ok(new_deck_id)
}