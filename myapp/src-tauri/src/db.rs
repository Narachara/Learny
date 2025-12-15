use tauri::path::BaseDirectory;
use tauri::Manager;
use rusqlite::{params, Connection};
use shared::models::*;
use serde_json;
use std::fs;
use std::path::PathBuf;
use tauri_plugin_dialog::{ DialogExt, FileDialogBuilder, FilePath };
use futures::channel::oneshot;

pub fn open_db(app: &tauri::AppHandle) -> Result<Connection, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    // ⭐ CRITICAL LINE — create directory
    fs::create_dir_all(&app_data_dir)
        .map_err(|e| format!("failed to create app data dir: {}", e))?;

    let db_path = app_data_dir.join("cards.db");

    Connection::open(db_path).map_err(|e| e.to_string())
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
            card_id INTEGER NOT NULL,
            side TEXT NOT NULL,
            position INTEGER NOT NULL,
            block_type TEXT NOT NULL,
            content TEXT NOT NULL,
            FOREIGN KEY (card_id) REFERENCES card(id) ON DELETE CASCADE
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
        let content: String = row.get(2)?; //retrieves exactly this string, byte-for-byte: {"type":"Text","value":"Transform each sentence...\n\nYou finish..."}

        let block: Block = serde_json::from_str(&content).unwrap(); // When deserialized, it becomes: Block::Text { value: "Hello" } and this is based on the tag. So the Tag decides what block tyoe the string gets serialized into :=)
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
                serde_json::to_string(block).unwrap() // Becomes json because the block enum has serialze
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

// Delete operations

fn delete_file_from_app_data(
    app: &tauri::AppHandle,
    virtual_path: &str,
) -> Result<(), String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    let full_path = app_data_dir.join(virtual_path);

    if !full_path.exists() {
        // already gone → not an error
        return Ok(());
    }

    std::fs::remove_file(&full_path)
        .map_err(|e| format!("Failed to delete {:?}: {}", full_path, e))?;

    Ok(())
}

#[tauri::command]
pub fn delete_card(app: tauri::AppHandle, id: i64) -> Result<(), String> {
    let conn = open_db(&app)?;

    conn.execute("PRAGMA foreign_keys = ON;", [])
        .map_err(|e| e.to_string())?;

    // Collect file paths
    let mut stmt = conn
        .prepare("SELECT content FROM block WHERE card_id = ?1")
        .map_err(|e| e.to_string())?;

    let paths: Vec<String> = stmt
        .query_map(rusqlite::params![id], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();

    // Delete files (best effort)
    for path in &paths {
        let path = std::path::Path::new(path);
        if path.exists() {
            if let Err(err) = std::fs::remove_file(path) {
                eprintln!("Failed to delete file {:?}: {}", path, err);
            }
        }
    }

    // Delete card (blocks cascade)
    let affected = conn.execute(
        "DELETE FROM card WHERE id = ?1",
        rusqlite::params![id],
    )
    .map_err(|e| e.to_string())?;

    if affected == 0 {
        return Err(format!("No card found with id {}", id));
    }

    Ok(())
}


#[tauri::command]
pub async fn download_file(
    app: tauri::AppHandle,
    virtual_path: String,
) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir()
        .map_err(|e| e.to_string())?;

    let source = app_data_dir.join(&virtual_path);

    let Some(file_name) = source.file_name() else {
        return Err("Invalid path".into());
    };

    let (tx, rx) = oneshot::channel();

    FileDialogBuilder::new(app.dialog().clone())
        .set_file_name(file_name.to_string_lossy())
        .save_file(move |file| {
            let _ = tx.send(file);
        });

    let dest = rx.await.map_err(|e| e.to_string())?;

    let Some(FilePath::Path(dest_path)) = dest else {
        // user cancelled → not an error
        return Ok(());
    };

    std::fs::copy(&source, &dest_path)
        .map_err(|e| e.to_string())?;

    Ok(())
}