use dioxus::prelude::*;
use shared::models::*;
use crate::app::Route;
use crate::components::{ CreateDeck };
use crate::tauri_api::{ init_db, get_decks };



#[component]
pub fn DeckList() -> Element {
    let nav = navigator();
    let mut creating = use_signal(|| false);

    // state for decks loaded from DB
    let mut decks = use_signal(|| Vec::<Deck>::new());

    // Initialize database + load decks once on mount

    use_future(move || async move {
        init_db().await;
        let loaded = get_decks().await;
        decks.set(loaded);
    });

    let deck_views: Vec<(i64, String)> = decks
        .read()
        .iter()
        .map(|d| (d.id, d.name.clone()))
        .collect();

    rsx! {
        div { class: "deck-list",

            h1 { "Select a Deck" }
            // Render all decks from state
            for (id, name) in deck_views {
                button {
                    key: "{id}",
                    class: "deck-item",
                    onclick: move |_| {
                        nav.push(Route::CardListPage { id: id });
                    },
                    "{name}"
                }
            }

            button {
                class: "add-deck-button",
                onclick: move |_| creating.set(true),
                "Add deck"
            }

            if *creating.read() {
                CreateDeck {
                    on_done: move |_| {
                        creating.set(false);

                        // reload deck list after adding new deck
                        spawn(async move {
                            let loaded = get_decks().await;
                            decks.set(loaded);
                        });
                    }
                }
            }
        }
    }
}
