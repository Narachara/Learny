use wasm_bindgen::{prelude::*};
use serde::{Serialize, de::DeserializeOwned, Deserialize};
use serde_wasm_bindgen;
use shared::models::{Deck, Card, Block};
use wasm_bindgen::JsValue;
use shared::FileResponse;

/// Bind to Tauri’s real invoke()
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(
        js_namespace = ["window", "__TAURI__", "core"],
        js_name = invoke
    )]
    async fn invoke_raw(cmd: &str, args: JsValue) -> JsValue;

}


pub async fn tauri<T, A>(cmd: &str, args: A) -> T
where
    T: DeserializeOwned,
    A: Serialize,
{
    let js_args = serde_wasm_bindgen::to_value(&args).unwrap();
    let raw = invoke_raw(cmd, js_args).await;
    serde_wasm_bindgen::from_value(raw).unwrap()
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteCardArgs {
    id: i64,
}

pub async fn delete_card(id: i64) {
    let _: () = tauri("delete_card", DeleteCardArgs { id }).await;
}



//
// ─────────────────────────────────────────────
//   Plugin Test
// ─────────────────────────────────────────────
//


pub async fn pick_image() -> String {
    let ret: Option<FileResponse> =
        tauri("plugin:bliet|pick_image", ()).await;

    match ret {
        Some(image) => image.path,
        None => String::new(), // ← user cancelled
    }
}

pub async fn pick_archive() -> String {
    let ret: Option<FileResponse> =
        tauri("plugin:bliet|pick_archive", ()).await;

    match ret {
        Some(archive) => archive.path,
        None => String::new(), // ← user cancelled
    }
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DownloadFileArgs {
    virtual_path: String,
}


pub async fn download_file(path: String) -> String {
    let ret: Option<FileResponse> =
        tauri("download_file", DownloadFileArgs { virtual_path: path } ).await;

    match ret {
        Some(archive) => archive.path,
        None => String::new(), // ← user cancelled
    }
}


//
// ─────────────────────────────────────────────
//   Import Export
// ─────────────────────────────────────────────
//

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ExportDeckArgs {
    deck_id: i64,
}


pub async fn export_deck(deck_id: i64) {
    let _: () = tauri("export_deck", ExportDeckArgs { deck_id } ).await;
}


pub async fn import_deck() -> i64 {
    tauri("import_deck", ()).await
}


//
// ─────────────────────────────────────────────
//   Commands
// ─────────────────────────────────────────────
//

pub async fn init_db() {
    // backend returns Result<(), String>
    // but Android actually returns raw `null`
    let _: () = tauri("init_db", ()).await;
}

//
// Decks
//

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AddDeckArgs {
    name: String,
}

pub async fn add_deck(name: String) -> i64 {
    // backend: Result<i64, String>
    tauri("add_deck", AddDeckArgs { name }).await
}

pub async fn get_decks() -> Vec<Deck> {
    // backend: Result<Vec<Deck>, String>
    tauri("get_decks", ()).await
}

//
// Cards
//

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddCardArgs {
    pub deck_id: i64,
    pub name: String,
}

pub async fn add_card(deck_id: i64, name: String) -> i64 {
    // backend: add_card(app, deck_id, name) → Result<i64, String>
    tauri("add_card", AddCardArgs { deck_id, name }).await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetCardArgs {
    id: i64,
}

pub async fn get_card(id: i64) -> Card {
    tauri("get_card", GetCardArgs { id }).await
}

//
// Save blocks
//

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SaveCardBlocksArgs<'a> {
    card_id: i64,
    front: &'a Vec<Block>,
    back: &'a Vec<Block>,
}

pub async fn save_card_blocks(card_id: i64, front: &Vec<Block>, back: &Vec<Block>) {
    let _: () = tauri(
        "save_card_blocks",
        SaveCardBlocksArgs { card_id, front, back }
    ).await;
}

//
// Update card name
//

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateCardNameArgs {
    id: i64,
    name: String,
}

pub async fn update_card_name(id: i64, name: String) {
    let _: () = tauri(
        "update_card_name",
        UpdateCardNameArgs { id, name }
    ).await;
}



#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GetCardsArgs {
    deck_id: i64,
}


pub async fn get_cards(deck_id: i64) -> Vec<Card> {
    tauri(
        "get_cards",
        GetCardsArgs {deck_id}
    ).await
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteDeckArgs {
    deck_id: i64,
}


pub async fn delete_deck(deck_id: i64) {
    let _: () = tauri( 
        "delete_deck",
        DeleteDeckArgs {deck_id}
    ).await;
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateCardScoreArgs {
    card_id: i64,
    correct: bool,
}


pub async fn update_score(card_id: i64, correct: bool) -> Card {
    tauri(
        "update_score",
        UpdateCardScoreArgs {card_id, correct}
    ).await
}