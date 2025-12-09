use dioxus::prelude::*;
use shared::models::*;

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

pub fn render_block(block: &Block) -> Element {
    match block {
        Block::Text { value } => { rsx!(p { class: "block-text", "{value}" }) }

        Block::Math { value } => { rsx!(MathBlock { value: value.clone() }) }

        Block::Image { src } => {
            let url = if cfg!(target_os = "android") {
                format!("http://appimg.localhost/{}", src)
            } else {
                format!("appimg://{}", src)
            };

            web_sys::console::log_1(&format!("IMG FINAL SRC = {}", url).into());

            rsx!(img {
                class: "block-image",
                src: "{url}",
            })
        }

        Block::File { name, path } => { rsx!(p { "{name} {path}" }) }
    }
}
