use shared::models::*;
use dioxus::prelude::*;

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
                p { "Preview:" }
                div { class: "block-math", dangerous_inner_html: "{value}" }
                button {
                    onclick: move |_| on_remove.call(()),
                    "Remove"
                }
            }
        ),

        Block::Image { src } => rsx!(
            div { class: "block-editor image-editor",
                input {
                    value: "{src}",
                    oninput: move |evt| {
                        on_update.call(
                            Block::Image { src: evt.value().to_string() }
                        );
                    }
                }
                img { class: "preview", src: "{src}" }
                button {
                    onclick: move |_| on_remove.call(()),
                    "Remove"
                }
            }
        ),

        Block::File { path } => rsx!(
            div { class: "block-editor file-editor",
                p { "File Stored" }
                button {
                    onclick: move |_| on_remove.call(()),
                    "Remove"
                }
            }
        ),
    }
}