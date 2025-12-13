mod db;
use tauri::http;
use tauri::Manager;
use mime_guess;
use urlencoding;
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
    let mut builder = tauri::Builder::default();

    builder = builder.setup(|app| {
        #[cfg(debug_assertions)] // only include this code on debug builds
        {
            let window = app.get_webview_window("main").unwrap();
            window.open_devtools();
            window.close_devtools();
        }
        Ok(())
    });

    builder = builder.register_uri_scheme_protocol("appimg", |_ctx, request| {
        let raw_path = request.uri().path();
        let decoded = urlencoding::decode(raw_path).unwrap();

        println!("RAW URI PATH     : {}", raw_path);
        println!("DECODED URI PATH : {}", decoded);

        
        // Remove leading slash
        let fs_path = &raw_path[1..];
        let file_path = std::path::PathBuf::from(fs_path);
        
        println!("FS PATH          : {}", file_path.display());
        println!("EXISTS?          : {}", file_path.exists());
        
        match std::fs::read(&file_path) {
            Ok(bytes) => {
                let mime = mime_guess::from_path(&file_path).first_or_octet_stream().to_string();

                http::Response
                    ::builder()
                    .header("Content-Type", mime)
                    .status(200)
                    .body(bytes)
                    .unwrap()
            }
            Err(e) => {
                http::Response
                    ::builder()
                    .status(404)
                    .body(format!("missing file: {}", e).into_bytes())
                    .unwrap()
            }
        }
    });

    builder
        .plugin(tauri_plugin_bliet::init())
        .plugin(tauri_plugin_dialog::init())
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
