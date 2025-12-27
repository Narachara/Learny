use dioxus::prelude::*;
use urlencoding::encode;
use shared::models::*;
use crate::tauri_api::{ download_file };


#[component]
pub fn MathBlock(value: String) -> Element {
    rsx!(div {
        class: "block-math",
        dangerous_inner_html: "{value}",

        onmounted: move |_| {
            let js = r#"setTimeout(() => {window.renderMath && window.renderMath();}, 50);"#;
            let _ = dioxus::document::eval(js);
        },
    })
}

fn appimg_url_from_virtual_path(virtual_path: &str) -> String {
    let encoded = virtual_path
        .split('/')
        .map(|seg| urlencoding::encode(seg))
        .collect::<Vec<_>>()
        .join("/");

    // TODO:
    // this check doesnt work
    // Ich kann es sonst auch einfach beim kompilieren jeweils schnell ändern
    // dann kann ich es umstellen
    // format!("http://appimg.localhost/{}", encoded) // for windows and android
    format!("appimg://{}", encoded) // for mac and linux
}

pub fn render_block(block: &Block) -> Element {
    match block {
        Block::Text { value } => { rsx!(p { class: "block-text", "{value}" }) }

        Block::Math { value } => { rsx!(MathBlock { value: value.clone() }) }

        Block::Image { src } => {
            let url = appimg_url_from_virtual_path(&src);
            rsx!(img {
                class: "block-image",
                src: "{url}",
            })
        }

        Block::File { path } => {
            let path0: String = path.clone();

            rsx!(
                    button {
                        class: "button button-primary",
                        onclick: move |_| {
                            let path = path0.clone();
                            spawn(async move {
                                let _ = download_file(path).await;
                            });
                        },
                        "Download Exercise File"
                    }
                )
        }
    }
}
