mod components;
mod app;
mod tauri_api;

use app::App;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).unwrap();
    launch(App);
}