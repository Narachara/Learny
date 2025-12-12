mod db;
use tauri::http;
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
    let mut builder = tauri::Builder::default();

    //
    // ────────────────────────────────────────────────────────────────
    // ANDROID ASYNC PROTOCOL HANDLER (http://appimg.localhost/<path>)
    // ────────────────────────────────────────────────────────────────
    //
    builder = builder.register_asynchronous_uri_scheme_protocol(
        "appimg",
        |_ctx, request, responder| {
            // This line proves if the handler is running
            println!("ANDROID/ASYNC PROTOCOL HIT → {}", request.uri());

            tauri::async_runtime::spawn(async move {
                // Example request: http://appimg.localhost/storage/emulated/0/.../image.png
                let uri = request.uri().to_string();
                println!("[URI] {}", uri);

                // Strip "http://appimg.localhost/"
                let without_prefix =
                    uri.replacen("http://appimg.localhost/", "", 1);

                println!("[WITHOUT PREFIX] {}", without_prefix);

                // Decode URL encoding
                let decoded =
                    urlencoding::decode(&without_prefix).unwrap_or_default();
                println!("[DECODED] {}", decoded);

                let file_path = std::path::PathBuf::from(decoded.to_string());
                println!("[FILE PATH] {:?}", file_path);

                match std::fs::read(&file_path) {
                    Ok(bytes) => {
                        let mime = mime_guess::from_path(&file_path)
                            .first_or_octet_stream()
                            .to_string();

                        println!("[OK] MIME={} SIZE={} bytes", mime, bytes.len());

                        responder.respond(
                            http::Response::builder()
                                .header("Content-Type", mime)
                                .status(200)
                                .body(bytes)
                                .unwrap(),
                        );
                    }
                    Err(e) => {
                        println!("[ERROR] {}", e);
                        responder.respond(
                            http::Response::builder()
                                .status(404)
                                .body(format!("missing file: {}", e).into_bytes())
                                .unwrap(),
                        );
                    }
                }
            });
        },
    );

    //
    // ────────────────────────────────────────────────────────────────
    // DESKTOP PROTOCOL HANDLER (appimg:///<path>)
    // ────────────────────────────────────────────────────────────────
    //
    builder = builder.register_uri_scheme_protocol("appimg", |_ctx, request| {
        println!("DESKTOP PROTOCOL HIT → {}", request.uri());

        let raw_path = request.uri().path(); // e.g. "/Users/.../image.png"
        println!("[DESKTOP PATH] {}", raw_path);

        // Remove leading slash
        let fs_path = &raw_path[1..];
        let file_path = std::path::PathBuf::from(fs_path);
        println!("[FILE PATH] {:?}", file_path);

        match std::fs::read(&file_path) {
            Ok(bytes) => {
                let mime = mime_guess::from_path(&file_path)
                    .first_or_octet_stream()
                    .to_string();

                println!("[OK] MIME={} SIZE={} bytes", mime, bytes.len());

                http::Response::builder()
                    .header("Content-Type", mime)
                    .status(200)
                    .body(bytes)
                    .unwrap()
            }
            Err(e) => {
                println!("[ERROR] {}", e);

                http::Response::builder()
                    .status(404)
                    .body(format!("missing file: {}", e).into_bytes())
                    .unwrap()
            }
        }
    });

    //
    // ────────────────────────────────────────────────────────────────
    //  REMAINDER: PLUGIN + YOUR COMMANDS
    // ────────────────────────────────────────────────────────────────
    //
    builder
        .plugin(tauri_plugin_bliet::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            init_db,
            add_deck,
            get_decks,
            add_card,
            get_cards,
            get_card,
            save_card_blocks,
            update_card_name
        ])
        .run(tauri::generate_context!())
        .expect("error running app");
}