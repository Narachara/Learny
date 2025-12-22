use dioxus::prelude::*;
use shared::models::*;
use crate::app::Route;
use crate::components::{ CreateDeck };
use crate::tauri_api::{ init_db, get_decks, export_deck, import_deck };

#[component]
pub fn DeckList() -> Element {
    let nav = navigator();
    let mut creating = use_signal(|| false);
    let mut decks = use_signal(|| Vec::<Deck>::new());

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
        div { class: "page",

            div { class: "deck-list",

                h1 { "Select a Deck" }

                for (id, name) in deck_views {

                    div { class: "deck-card",

                        button {
                            class: "deck-main",
                            onclick: move |_| {
                                nav.push(Route::CardListPage { id });
                            },
                            "{name}"
                        }


                        div { class: "deck-actions",
                            button {
                                class: "button",
                                onclick: move |_| {
                                    spawn(async move {
                                        export_deck(id).await;
                                    });
                                },
                                "Export"
                            }

                        // TODO: 
                        // Delete deck options
                            button {
                                class: "button button-danger",
                                "Delete Deck"
                            }
                        }
                    }
                }

                div { class: "deck-global-actions",
                    button {
                        class: "button",
                        onclick: move |_| creating.set(true),
                        "Add deck"
                    }

                    button {
                        class: "button",
                        onclick: move |_| {
                            spawn(async move {
                                let new_deck_id = import_deck().await;
                                if new_deck_id > 0 {
                                    nav.push(Route::CardListPage { id: new_deck_id });
                                }
                                let loaded = get_decks().await;
                                decks.set(loaded);
                            });
                        },
                        "Import deck"
                    }
                }

                if *creating.read() {
                    CreateDeck {
                        on_done: move |_| {
                            creating.set(false);
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
}
