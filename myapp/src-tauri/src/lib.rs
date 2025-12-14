mod db;
use tauri::http;
use tauri::{Manager, AppHandle};
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
    download_file,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
    .register_uri_scheme_protocol("appimg", |_ctx, request| {
        println!("🔥 APPIMG HANDLER HIT 🔥");
        // FULL URI: appimg://localhost/Files/5755e5a2-91de-4077-b610-f531e8fdddc3.png
        println!("FULL URI: {}", request.uri());

        let uri = request.uri();

        let raw_path = uri.path();
        let decoded = urlencoding::decode(raw_path).unwrap();

        let mut virtual_path = decoded.trim_start_matches('/').to_string();

        if let Some(host) = uri.host() {
            if host != "localhost" && !virtual_path.starts_with(&format!("{}/", host)) {
                virtual_path = format!("{}/{}", host, virtual_path);
            }
        }

        let app_data_dir = _ctx.app_handle().path().app_data_dir().unwrap();
        let full_path = app_data_dir.join(&virtual_path);

        println!("APP DATA DIR : {}", app_data_dir.display());
        println!("VIRTUAL PATH : {}", virtual_path);
        println!("FULL FS PATH : {}", full_path.display());
        println!("EXISTS       : {}", full_path.exists());

        match std::fs::read(&full_path) {
            Ok(bytes) => {
                let mime = mime_guess::from_path(&full_path).first_or_octet_stream().to_string();

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
    })
    .setup(|app| {
        #[cfg(debug_assertions)] // only include this code on debug builds
        {
            let window = app.get_webview_window("main").unwrap();
            window.open_devtools();
            window.close_devtools();
        }
        Ok(())
    })
    .plugin(tauri_plugin_bliet::init()).plugin(tauri_plugin_dialog::init())
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
                download_file,
            ]
        )
        .run(tauri::generate_context!())
        .expect("error running app");
}
