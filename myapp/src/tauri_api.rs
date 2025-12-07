use wasm_bindgen::prelude::*;
use serde::{Serialize, de::DeserializeOwned, Deserialize};
use serde_wasm_bindgen;
use shared::models::{Deck, Card, Block};
use wasm_bindgen::JsValue;
use web_sys::console;

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

    console::log_1(&JsValue::from_str(&format!(
        "ARGS for {} = {:?}",
        cmd, js_args
    )));

    let raw = invoke_raw(cmd, js_args).await;

    web_sys::console::log_1(
        &JsValue::from_str(&format!("RAW RESPONSE({}): {:?}", cmd, raw))
    );
    serde_wasm_bindgen::from_value(raw).unwrap()
}


//
// ─────────────────────────────────────────────
//   Plugin Test
// ─────────────────────────────────────────────
//


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PingRequest {
    value: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PingArgs {
    payload: PingRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingResponse {
    pub value: String,
}

pub async fn ping() {
    let payload = PingArgs {
        payload: PingRequest {
            value: "Hello".into(),
        }
    };

    let resp: PingResponse = tauri("plugin:bliet|ping", payload).await;
    web_sys::console::log_1(&format!("PING RESPONSE: {:?}", resp).into());
}

// define a Struct for the image
#[derive(Debug, Serialize, Deserialize)]
pub struct ImageResponse {
    pub path: String,
}


pub async fn pick_image() -> String {
    let ret: ImageResponse = tauri("plugin:bliet|pick-image", ()).await;
    ret.path
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