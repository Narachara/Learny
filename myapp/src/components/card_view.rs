use dioxus::prelude::*;
use shared::models::{Card, Block};
use crate::components::block_view::render_block;
use crate::components::card_list_page::CardListPage;
use crate::components::{CardEditorEdit};
use crate::app::Route;
use crate::tauri_api::{ get_card };


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
pub fn CardView(id: i64) -> Element {
    let mut show_answer = use_signal(|| false);
    let nav = navigator();

    // let card = Card {
    //     id: 1,
    //     deck_id: 1,
    //     created_at: 0,
    //     times_seen: 0,
    //     times_correct: 0,
    //     tags: None,
    //     name: "myfirst_card".into(),

    //     front_blocks: vec![
    //         Block::Text { value: "What is the derivative of sin(x)?".into() },
    //         Block::Math { value: r#"$$ V(s) = \max_{a} \left[ R(s, a) + \gamma \sum_{s'} P(s' \mid s, a)\, V(s') \right]$$"#.into() },
    //         Block::Text { value: "Final note: derivative is periodic.".into() }, 
    //         Block::Text { value: r#"Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet."#.into() }
    //     ],
    //     back_blocks: vec![Block::Text { value: "The answer is 42".into() }],
    //     };


    let mut card = use_signal( || Card::new_empty(id) );

    use_effect(move || {
        spawn(async move {
            let loaded = get_card(id).await;
            card.set(loaded);
        });
    });

    let card = card.read();

    rsx! {
        div { class: "card-list-page",

            h1 {"{&card.name}"}

            div { class: "card-surface",
                for block in &card.front_blocks {
                    { render_block(block) }
                }
            }

            button {
                class: "show-answer-btn",
                onclick: move |_| show_answer.set(true),
                "Show answer"
            }

            if *show_answer.read() {
                div { class: "answer-surface",
                    for block in &card.back_blocks {
                        { render_block(block) }
                    }
                }
            }
        }

        button {
    class: "edit-button",
    onclick: move |_| {
        nav.push(Route::CardEditorEdit { id: id });
    },
    "Edit Card"
}

        button {
            class: "back-button",
            onclick: move |_| { nav.push(Route::CardListPage { id: id }); },
            "Back"
        }
    }
}