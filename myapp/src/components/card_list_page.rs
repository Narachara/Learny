use dioxus::prelude::*;
use dioxus_router::prelude::*;
use shared::models::{ Deck, Block, Card };
use crate::app::Route;
use crate::tauri_api::{ get_cards };

#[component]
pub fn CardListPage(id: i64) -> Element {
    let nav = navigator();
    let mut cards = use_signal(|| Vec::<Card>::new());

    use_effect(move || {
        spawn(async move {
            let loaded = get_cards(id).await;
            cards.set(loaded);
        });
    });

    let card_views: Vec<(i64, String, u8)> = cards
        .read()
        .iter()
        .map(|c| (c.id, c.name.clone(), c.progress_percent()))
        .collect();

    rsx! {
        div { class: "card-list-page",

            h1 { "Cards in deck {id}" }

            div { class: "cards-container",

                for (card_id, card_name, progress) in card_views {
                        div { key: "{card_id}", class: "card-preview",

                        div { class: "card-main",
                            h2 { class: "card-title", "{card_name}" }

                            div { class: "card-progress",
                                div {
                                    class: "card-progress-bar",
                                    style: "width: {progress}%;"
                                }
                            }
                        }

                        button {
                            class: "card-open-button",
                            onclick: move |_| { nav.push(Route::CardView { id: card_id }); },
                            "Open"
                        }
                    }
                }
            }

            div { class: "cardlist-buttons",
                button {
                    class: "back-button",
                    onclick: move |_| { nav.push(Route::DeckList); },
                    "Back"
                }

                button {
                    class: "add-card-button",
                    // we use 1 for now but later we need to get the current deck id when 
                    // listing the cards
                    onclick: move |_| { nav.push(Route::CardEditorNew { deck_id: id } ); },
                    "Add Card"
                }
            }


        }
    }
}
