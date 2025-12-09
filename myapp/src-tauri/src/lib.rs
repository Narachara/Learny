mod db;
use tauri::Manager;
use tauri::http;
use std::borrow::Cow;
use mime_guess;
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
        .register_asynchronous_uri_scheme_protocol("appimg", |ctx, request, responder| {
            tauri::async_runtime::spawn(async move {
                let uri = request.uri().to_string();

                println!("ASYNC PROTOCOL CALLED WITH URI: {}", uri);

                // Remove the Android rewrite prefix:
                let path = uri.replacen("http://appimg.localhost/", "", 1);

                // URL decode the path
                let decoded = urlencoding::decode(&path).unwrap_or_default();
                let fs_path = decoded.to_string();

                println!("Decoded FS PATH = {}", fs_path);

                let file_path = std::path::PathBuf::from(&fs_path);

                let bytes = std::fs::read(&file_path).unwrap_or_else(|err| {
                    println!("ERROR reading file: {}", err);
                    Vec::new()
                });

                let mime = mime_guess::from_path(&file_path).first_or_octet_stream().to_string();

                responder.respond(
                    tauri::http::Response
                        ::builder()
                        .header("Content-Type", mime)
                        .status(200)
                        .body(bytes)
                        .unwrap()
                );
            });
        })
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
                update_card_name
            ]
        )
        .run(tauri::generate_context!())
        .expect("error running app");
}
