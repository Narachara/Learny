use dioxus::prelude::*;
use dioxus_router::prelude::*;
use shared::models::{Deck, Block, Card};
use crate::app::Route;
use crate::tauri_api::{get_cards};


#[component]
pub fn CardListPage(id: i64) -> Element {
    let nav = navigator();

    // Example cards:
    // let cards = vec![
    //     Card {
    //         id: 1,
    //         name: "Yea".into(),
    //         deck_id: id,
    //         front_blocks: vec![Block::Text { value: "What is Rust?".into() }],
    //         back_blocks: vec![Block::Text { value: "A programming language".into() }],
    //         created_at: 0,
    //         times_seen: 0,
    //         times_correct: 0,
    //         tags: None,
    //     },
    //     Card {
    //         id: 2,
    //         name: "Yea2".into(),
    //         deck_id: id,
    //         front_blocks: vec![Block::Text { value: "2 + 2 = ?".into() }],
    //         back_blocks: vec![Block::Math { value: "4".into() }],
    //         created_at: 0,
    //         times_seen: 0,
    //         times_correct: 0,
    //         tags: None,
    //     }
    // ];

    let mut cards = use_signal(|| Vec::<Card>::new());

    use_effect(move || {
        spawn(async move {
            let loaded = get_cards(id).await;
            cards.set(loaded);
        });
    });

    let card_views: Vec<(i64, String)> = cards
        .read()
        .iter()
        .map(|c| (c.id, c.name.clone()))
        .collect();



    rsx! {
        div { class: "card-list-page",

            h1 { "Cards in deck {id}" }

            div { class: "cards-container",

                for (card_id, card_name) in card_views {
                    div { key: "{card_id}", class: "card-preview",

                        h2 { class: "card-title", "{card_name}" }

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