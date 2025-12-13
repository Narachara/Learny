use dioxus::prelude::*;
use urlencoding::encode;
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


fn appimg_url_from_fs_path(src: &str) -> String {
    // Split into segments, encode each segment, then re-join with '/'
    // This keeps '/' as path separators (not %2F).
    let encoded_path = src
        .split('/')
        .map(|seg| encode(seg))
        .collect::<Vec<_>>()
        .join("/");

    // If src is absolute on Unix/macOS, it starts with '/', so we need 3 slashes total:
    // appimg:///Users/...
    format!("appimg://{}", encoded_path)
}

fn appimg_url_from_fs_path2(path: &str) -> String {
    // Preserve leading slash for absolute paths
    let is_absolute = path.starts_with('/');

    let encoded = path
        .split('/')
        .map(|seg| urlencoding::encode(seg))
        .collect::<Vec<_>>()
        .join("/");

    if is_absolute {
        format!("appimg:///{}", encoded.trim_start_matches('/'))
    } else {
        format!("appimg://{}", encoded)
    }
}


pub fn render_block(block: &Block) -> Element {
    match block {
        Block::Text { value } => { rsx!(p { class: "block-text", "{value}" }) }

        Block::Math { value } => { rsx!(MathBlock { value: value.clone() }) }

        Block::Image { src } => {
            let url = appimg_url_from_fs_path2(&src);

            // for android this is just enough:
            // let url = format!("http://appimg.localhost{}", src);
            web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&url));
            rsx!(img {
                class: "block-image",
                src: "{url}",
            })
        }

        Block::File { name, path } => { rsx!(p { "{name} {path}" }) }
    }
}
