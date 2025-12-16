use crate::db::{ get_card, open_db, get_deck };
use tauri::Manager;

#[derive(Serialize)]
pub struct DeckExport {
    pub export_version: u32,
    pub deck: Deck,
    pub cards: Vec<Card>,
}


pub fn get_card_ids(app: &AppHandle, deck_id: i64) -> Result<Vec<i64>, String> {
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

pub fn export_deck_cards(app: tauri::AppHandle, deck_id: i64) -> Result<Vec<Card>, String> {
    let card_ids = get_card_ids(&app, deck_id)?;

    let mut cards = Vec::new();
    for id in card_ids {
        cards.push(get_card(app.clone(), id)?);
    }

    Ok(cards)
}


pub fn export_deck_json(app: tauri::AppHandle, deck_id: i64) -> Result<String, String> {
    let deck = get_deck(app.clone(), deck_id)?;
    let cards = export_deck_cards(app, deck_id)?;

    let export = DeckExport {
        export_version: 1,
        deck,
        cards,
    };

    serde_json::to_string_pretty(&export)
        .map_err(|e| e.to_string())
}