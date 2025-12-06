use tauri::path::BaseDirectory;
use tauri::Manager;
use rusqlite::{params, Connection};
use shared::models::*;
use serde_json;

pub fn open_db(app: &tauri::AppHandle) -> Result<Connection, String> {
    let path = app
        .path()
        .resolve("cards.db", BaseDirectory::AppData)
        .map_err(|e| e.to_string())?;

    Connection::open(path).map_err(|e| e.to_string())
}


#[tauri::command]
pub fn init_db(app: tauri::AppHandle) -> Result<(), String> {
    let conn = open_db(&app)?;

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS deck (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            created_at INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS card (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            deck_id INTEGER NOT NULL REFERENCES deck(id),
            name TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            times_seen INTEGER NOT NULL DEFAULT 0,
            times_correct INTEGER NOT NULL DEFAULT 0,
            tags TEXT
        );

        CREATE TABLE IF NOT EXISTS block (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            card_id INTEGER NOT NULL REFERENCES card(id),
            side TEXT NOT NULL,
            position INTEGER NOT NULL,
            block_type TEXT NOT NULL,
            content TEXT NOT NULL
        );
        "
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn add_deck(app: tauri::AppHandle, name: String) -> Result<i64, String> {
    let conn = open_db(&app)?;
    let now = chrono::Utc::now().timestamp();

    conn.execute(
        "INSERT INTO deck (name, created_at) VALUES (?1, ?2)",
        params![name, now],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn get_decks(app: tauri::AppHandle) -> Result<Vec<Deck>, String> {
    let conn = open_db(&app)?;

    let mut stmt = conn
        .prepare("SELECT id, name, created_at FROM deck ORDER BY id DESC")
        .map_err(|e| e.to_string())?;

    let decks = stmt
        .query_map([], |row| {
            Ok(Deck {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                card_count: 0, // filled later
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;

    Ok(decks)
}



// Create
#[tauri::command]
pub fn add_card(app: tauri::AppHandle, deck_id: i64, name: String) -> Result<i64, String> {
    let conn = open_db(&app)?;
    let now = chrono::Utc::now().timestamp();

    conn.execute(
        "INSERT INTO card (deck_id, name, created_at)
         VALUES (?1, ?2, ?3)",
        params![deck_id, name, now],
    )
    .map_err(|e| e.to_string())?;

    Ok(conn.last_insert_rowid())
}


#[tauri::command]
pub fn get_cards(app: tauri::AppHandle, deck_id: i64) -> Result<Vec<Card>, String> {
    let conn = open_db(&app)?;

    let mut stmt = conn.prepare(
        "
        SELECT id, deck_id, name, created_at, times_seen, times_correct, tags
        FROM card
        WHERE deck_id = ?
        ORDER BY created_at DESC
        "
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map([deck_id], |row| {
        Ok(Card {
            id: row.get(0)?,
            deck_id: row.get(1)?,
            name: row.get(2)?,
            created_at: row.get(3)?,
            times_seen: row.get(4)?,
            times_correct: row.get(5)?,
            tags: row.get(6)?,
            front_blocks: vec![],
            back_blocks: vec![],
        })
    })
    .map_err(|e| e.to_string())?;

    let mut cards = Vec::new();
    for card in rows {
        cards.push(card.map_err(|e| e.to_string())?);
    }

    Ok(cards)
}

#[tauri::command]
pub fn get_card(app: tauri::AppHandle, id: i64) -> Result<Card, String> {
    let conn = open_db(&app)?;

    // load card
    let card = conn.query_row(
        "
        SELECT id, deck_id, name, created_at, times_seen, times_correct, tags
        FROM card
        WHERE id = ?
        ",
        [id],
        |row| {
            Ok(Card {
                id: row.get(0)?,
                deck_id: row.get(1)?,
                name: row.get(2)?,
                created_at: row.get(3)?,
                times_seen: row.get(4)?,
                times_correct: row.get(5)?,
                tags: row.get(6)?,
                front_blocks: vec![],
                back_blocks: vec![],
            })
        },
    )
    .map_err(|e| e.to_string())?;

    let mut front = Vec::<Block>::new();
    let mut back = Vec::<Block>::new();

    let mut stmt = conn.prepare(
        "
        SELECT side, block_type, content
        FROM block
        WHERE card_id = ?
        ORDER BY position ASC
        "
    ).map_err(|e| e.to_string())?;

    let rows = stmt.query_map([id], |row| {
        let side: String = row.get(0)?;
        let content: String = row.get(2)?;

        let block: Block = serde_json::from_str(&content).unwrap();
        Ok((side, block))
    }).map_err(|e| e.to_string())?;

    for row in rows {
        let (side, block) = row.map_err(|e| e.to_string())?;
        if side == "front" {
            front.push(block);
        } else {
            back.push(block);
        }
    }

    Ok(Card {
        front_blocks: front,
        back_blocks: back,
        ..card
    })
}

#[tauri::command]
pub fn save_card_blocks(
    app: tauri::AppHandle,
    card_id: i64,
    front: Vec<Block>,
    back: Vec<Block>,
) -> Result<(), String> {

    let conn = open_db(&app)?;

    conn.execute("DELETE FROM block WHERE card_id = ?", [card_id])
        .map_err(|e| e.to_string())?;

    for (i, block) in front.iter().enumerate() {
        conn.execute(
            "
            INSERT INTO block (card_id, side, position, block_type, content)
            VALUES (?1, 'front', ?2, ?3, ?4)
            ",
            params![
                card_id,
                i as i64,
                block.block_type(),
                serde_json::to_string(block).unwrap()
            ],
        )
        .map_err(|e| e.to_string())?;
    }

    for (i, block) in back.iter().enumerate() {
        conn.execute(
            "
            INSERT INTO block (card_id, side, position, block_type, content)
            VALUES (?1, 'back', ?2, ?3, ?4)
            ",
            params![
                card_id,
                i as i64,
                block.block_type(),
                serde_json::to_string(block).unwrap()
            ],
        )
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}



#[tauri::command]
pub fn update_card_name(app: tauri::AppHandle, id: i64, name: String) -> Result<(), String> {
    let conn = open_db(&app)?;
    conn.execute(
        "UPDATE card SET name = ?1 WHERE id = ?2",
        params![name, id]
    ).map_err(|e| e.to_string())?;
    Ok(())
}


// #[tauri::command]
// async fn pick_image(app: tauri::AppHandle) -> Result<String, String> {
//     crate::android::pick_image(app).await
// }