use shared::models::*;
use dioxus::prelude::*;
use crate::tauri_api::delete_block_from_app_data;

#[component]
pub fn BlockEditor(
    block: Block,
    on_update: EventHandler<Block>,
    on_remove: EventHandler<()>,
) -> Element {

    match block {
        Block::Text { value } => rsx!(
            div { class: "block-editor text-editor",
                textarea {
                    value: "{value}",
                    oninput: move |evt| {
                        on_update.call(
                            Block::Text { value: evt.value().to_string() }
                        );
                    }
                }
                button {
                    onclick: move |_| on_remove.call(()),
                    "Remove"
                }
            }
        ),

        Block::Math { value } => rsx!(
            div { class: "block-editor math-editor",
                textarea {
                    value: "{value}",
                    oninput: move |evt| {
                        on_update.call(
                            Block::Math { value: evt.value().to_string() }
                        );
                    }
                }
                button {
                    onclick: move |_| on_remove.call(()),
                    "Remove"
                }
            }
        ),

        Block::Image { src } => rsx!(
            div { class: "block-editor image-editor",
                p {"Image saved"}
                button {
                    onclick: move |_| {
                        let src = src.clone();
                        let on_remove = on_remove.clone();

                        spawn(async move {
                            web_sys::console::log_1(
                                &wasm_bindgen::JsValue::from_str("Deleting image file…")
                            );

                            delete_block_from_app_data(src).await;

                            web_sys::console::log_1(
                                &wasm_bindgen::JsValue::from_str("Delete finished")
                            );

                            on_remove.call(());
                        });
                    },
                    "Remove"
                }
            }
        ),

        Block::File { path } => rsx!(
            div { class: "block-editor file-editor",
                p { "File Stored" }
                button {
                    onclick: move |_| {
                        let path = path.clone();
                        let on_remove = on_remove.clone();

                        spawn(async move {
                            web_sys::console::log_1(
                                &wasm_bindgen::JsValue::from_str("Deleting file…")
                            );

                            delete_block_from_app_data(path).await;

                            web_sys::console::log_1(
                                &wasm_bindgen::JsValue::from_str("Delete finished")
                            );

                            on_remove.call(());
                        });
                    },
                    "Remove"
                }

            }
        ),
    }
}