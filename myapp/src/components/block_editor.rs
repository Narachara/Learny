use shared::models::*;
use dioxus::prelude::*;
use crate::tauri_api::{delete_block_from_app_data, pick_image, pick_archive};

#[component]
pub fn BlockEditor(
    block: Block,
    on_update: EventHandler<Block>,
    on_remove: EventHandler<()>,
    on_insert_above: EventHandler<InsertBlockKind>,
    on_insert_below: EventHandler<InsertBlockKind>,
) -> Element {

    let insert_menu = |handler: EventHandler<InsertBlockKind>| rsx!(
        div { class: "insert-menu",
            button { onclick: move |_| handler.call(InsertBlockKind::Text), "＋ Text" }
            button { onclick: move |_| handler.call(InsertBlockKind::Math), "＋ Math" }
            button { onclick: move |_| handler.call(InsertBlockKind::Image), "＋ Image" }
            button { onclick: move |_| handler.call(InsertBlockKind::File), "＋ File" }
        }
    );

    match block {
        Block::Text { value } => rsx!(
            div { class: "block-editor text-editor",

                {insert_menu(on_insert_above.clone())}

                textarea {
                    value: "{value}",
                    oninput: move |evt| {
                        on_update.call(Block::Text {
                            value: evt.value().to_string()
                        });
                    }
                }

                {insert_menu(on_insert_below.clone())}

                button {
                    onclick: move |_| on_remove.call(()),
                    "🗑 Remove"
                }
            }
        ),

        Block::Math { value } => rsx!(
            div { class: "block-editor math-editor",

                {insert_menu(on_insert_above.clone())}

                textarea {
                    value: "{value}",
                    oninput: move |evt| {
                        on_update.call(Block::Math {
                            value: evt.value().to_string()
                        });
                    }
                }

                {insert_menu(on_insert_below.clone())}

                button {
                    onclick: move |_| on_remove.call(()),
                    "🗑 Remove"
                }
            }
        ),

        Block::Image { src } => rsx!(
            div { class: "block-editor image-editor",

                {insert_menu(on_insert_above.clone())}

                p { "Image saved" }

                {insert_menu(on_insert_below.clone())}

                button {
                    onclick: move |_| {
                        let src = src.clone();
                        let on_remove = on_remove.clone();
                        spawn(async move {
                            delete_block_from_app_data(src).await;
                            on_remove.call(());
                        });
                    },
                    "🗑 Remove Image"
                }
            }
        ),

        Block::File { path } => rsx!(
            div { class: "block-editor file-editor",

                {insert_menu(on_insert_above.clone())}

                p { "File stored" }

                {insert_menu(on_insert_below.clone())}

                button {
                    onclick: move |_| {
                        let path = path.clone();
                        let on_remove = on_remove.clone();
                        spawn(async move {
                            delete_block_from_app_data(path).await;
                            on_remove.call(());
                        });
                    },
                    "🗑 Remove File"
                }
            }
        ),
    }
}