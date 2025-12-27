use dioxus::prelude::*;
use shared::models::*;
use crate::app::Route;
use crate::components::{ CreateDeck };
use crate::tauri_api::{ init_db, get_decks, export_deck, import_deck, get_cards, delete_card, delete_deck, rename_deck };


#[component]
pub fn DeleteDeck(deck_id: i64, on_done: EventHandler<()>) -> Element {
    rsx! {
        div { class: "delete-card",

            h1 { "Sure you want to delete the deck forever?" }

            div { class: "delete-actions",

                button {
                    class: "button button-danger",
                    onclick: move |_| {
                        spawn(async move {
                            let _ = delete_deck(deck_id).await;
                            on_done.call(());
                        });
                    },
                    "YES"
                }

                button {
                    class: "button button-secondary",
                    onclick: move |_| on_done.call(()),
                    "NO"
                }
            }
        }
    }
}


#[component]
pub fn DeckList() -> Element {
    let nav = navigator();
    let mut creating = use_signal(|| false);
    let mut deleting: Signal<Option<i64>> = use_signal(|| None);
    let mut decks = use_signal(|| Vec::<Deck>::new());
    let mut renaming: Signal<Option<i64>> = use_signal(|| None);
    let mut rename_value = use_signal(String::new);

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

                            button {
                                class: "button button-danger",
                                onclick: move |_| deleting.set(Some(id)),
                                "Delete Deck"
                            }

                            button {
                                class: "button",
                                onclick: move |_| {
                                    renaming.set(Some(id));
                                    rename_value.set(name.clone());
                                },
                                "Rename"
                            }
                        }
                        if deleting.read().as_ref() == Some(&id) {
                            DeleteDeck {
                                deck_id: id,
                                on_done: move |_| {
                                    deleting.set(None);
                                    spawn(async move {
                                        let loaded = get_decks().await;
                                        decks.set(loaded);
                                    });
                                }
                            }
                        }
                    }

                    if renaming.read().as_ref() == Some(&id) {
                        div { class: "rename-deck",

                            input {
                                class: "rename_input",
                                value: "{rename_value}",
                                autofocus: true,
                                oninput: move |e| rename_value.set(e.value()),
                                onkeydown: move |e| {
                                    if e.key() == Key::Enter {
                                        let new_name = rename_value.read().clone();
                                        renaming.set(None);

                                        spawn(async move {
                                            let _ = rename_deck(new_name, id).await;
                                            let loaded = get_decks().await;
                                            decks.set(loaded);
                                        });
                                    }
                                }
                            }

                            div { class: "rename-actions",

                                button {
                                    class: "button button-primary",
                                    onclick: move |_| {
                                        let new_name = rename_value.read().clone();
                                        renaming.set(None);

                                        spawn(async move {
                                            let _ = rename_deck(new_name, id).await;
                                            let loaded = get_decks().await;
                                            decks.set(loaded);
                                        });
                                    },
                                    "Save"
                                }

                                button {
                                    class: "button button-secondary",
                                    onclick: move |_| renaming.set(None),
                                    "Cancel"
                                }
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
