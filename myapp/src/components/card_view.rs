use dioxus::prelude::*;
use shared::models::{ Card };
use crate::components::block_view::render_block;
use crate::components::card_list_page::CardListPage;
use crate::components::{ CardEditorEdit };
use crate::app::Route;
use crate::tauri_api::{ get_card, delete_card };

fn card_score(times_known: u32, times_done: u32) -> f64 {
    if times_done == 0 {
        return f64::INFINITY;
    }

    let p = (times_known as f64) / (times_done as f64);

    if p <= 0.0 {
        return f64::INFINITY;
    }

    -p.ln()
}

#[component]
pub fn DeleteCard(card_id: i64, on_done: EventHandler<()>) -> Element {
    rsx! {
        div { class: "delete-card",

            h1 { "Sure you want to delete the card forever?" }

            div { class: "delete-actions",

                button {
                    class: "button button-danger",
                    onclick: move |_| {
                        spawn(async move {
                            let _ = delete_card(card_id).await;
                            on_done.call(());
                        });
                    },
                    "YES"
                }

                button {
                    class: "button button-secondary",
                    onclick: move |_| {
                        on_done.call(());
                    },
                    "NO"
                }
            }
        }
    }
}

#[component]
pub fn CardView(id: i64) -> Element {
    let mut show_answer = use_signal(|| false);
    let mut deleting = use_signal(|| false);
    let nav = navigator();

    let mut card = use_signal(|| Card::new_empty(id));

    use_effect(move || {
        spawn(async move {
            let loaded = get_card(id).await;
            card.set(loaded);
        });
    });

    let card = card.read();
    let deck_id = card.deck_id;

    rsx! {
    div { class: "card-view",

        h1 { class: "card-title", "{card.name}" }

        // Study area
        div { class: "card-study",

            div { class: "card-surface",
                for block in &card.front_blocks {
                    { render_block(block) }
                }
            }

        if !*show_answer.read() {
            div { class: "show-answer-container",
                button {
                    class: "button button-primary",
                    onclick: move |_| show_answer.set(true),
                    "Show answer"
                }
            }
        }


            if *show_answer.read() {
                div { class: "answer-surface",
                    for block in &card.back_blocks {
                        { render_block(block) }
                    }
                }
            }
        }

        // Actions
        div { class: "card-actions",

            button {
                class: "button button-secondary",
                onclick: move |_| {
                    nav.push(Route::CardEditorEdit { id });
                },
                "Edit"
            }

            button {
                class: "button button-danger",
                onclick: move |_| deleting.set(true),
                "Delete"
            }

            button {
                class: "button button-secondary",
                onclick: move |_| {
                    nav.push(Route::CardListPage { id: deck_id });
                },
                "Back"
            }
        }
    }
        if *deleting.read() {
            DeleteCard {
                card_id: id,
                on_done: move |_| {
                    deleting.set(false);
                    nav.push(Route::CardListPage { id: deck_id });
                }
            }
        }

    }
}
