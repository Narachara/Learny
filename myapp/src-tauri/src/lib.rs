mod db;
use tauri::Manager;
use crate::db::{
    init_db,
    add_deck,
    get_decks,
    add_card,
    save_card_blocks,
    update_card_name,
    get_card,
    get_cards,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder
        ::default()
        .plugin(tauri_plugin_bliet::init()) // register m< plugin
        .invoke_handler(
            tauri::generate_handler![
                init_db,
                add_deck,
                get_decks,
                add_card,
                get_cards,
                get_card,
                save_card_blocks,
                update_card_name,
            ]
        )
        .run(tauri::generate_context!())
        .expect("error running app");
}
