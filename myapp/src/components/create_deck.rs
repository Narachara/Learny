use dioxus::prelude::*;
use crate::tauri_api::add_deck;

#[component]
pub fn CreateDeck(on_done: EventHandler<()>) -> Element {
    let mut deck_name = use_signal(|| "".to_string());

    rsx! {
        div { class: "create-deck",

            h1 { "Give your deck a name" }

            input {
                class: "deck-input",
                placeholder: "e.g. Calculus",
                value: "{deck_name}",
                oninput: move |ev| deck_name.set(ev.value()),
            }

            button {
                class: "button button-primary ",
                onclick: move |_| {
                    let name = deck_name.read().to_string();
                    if !name.is_empty() {
                        spawn(async move {
                            add_deck(name).await;
                            on_done.call(());
                        });
                    } else { on_done.call(())}
                },
                "Save"
            }
        }
    }
}