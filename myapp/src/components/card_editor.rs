use dioxus::prelude::*;
use shared::models::{Card, Block, Deck};
use crate::components::BlockEditor;
use crate::app::Route;
use crate::tauri_api::{get_card, add_card ,update_card_name, save_card_blocks, pick_image };


#[derive(Clone, PartialEq, Copy)]
pub enum EditorMode {
    New {
        deck_id: i64,
    },
    Edit {
        card_id: i64,
    },
}

#[component]
pub fn CardEditorNew(deck_id: i64) -> Element {
    rsx! {
        CardEditor {
            mode: EditorMode::New { deck_id },
        }
    }
}

#[component]
pub fn CardEditorEdit(id: i64) -> Element {
    rsx! {
        CardEditor {
            mode: EditorMode::Edit { card_id: id },
        }
    }
}

#[component]
pub fn CardEditor(mode: EditorMode) -> Element {
    let nav = navigator();

    // State for the card being edited
    let mut card = use_signal(|| None::<Card>);


    //
    // MODE-DEPENDENT INITIALIZATION
    //

    match mode {
        EditorMode::New { deck_id } => {
            // Create a fresh card right away
            card.set(Some(Card::new_empty(deck_id)));
        }
        EditorMode::Edit { card_id } => {
            // Load from DB once on mount
            use_effect(move || {
                spawn(async move {
                    let loaded = get_card(card_id).await;
                    card.set(Some(loaded));
                });
            });
        }
    }

    //
    // If card is not loaded yet (Edit mode), show a loading state
    //
    if card.read().is_none() {
        return rsx! {
            div { class: "loading",
                "Loading card..."
            }
        };
    }

    //
    // Destructure card – now it’s guaranteed to be Some
    //
    let c = card.read().as_ref().unwrap().clone();

    let mut card_name = use_signal(|| c.name.clone());
    let mut front_blocks = use_signal(|| c.front_blocks.clone());
    let mut back_blocks = use_signal(|| c.back_blocks.clone());

    //
    // RENDER
    //
    rsx! {
        div { class: "card-editor-page",

            h1 { 
                match mode {
                    EditorMode::New { .. } => "Create New Card",
                    EditorMode::Edit { .. } => "Edit Card",
                }
            }

            // Card name
            div { class: "card-field",
                label { "Card Name" }
                input {
                    value: "{card_name}",
                    oninput: move |evt| card_name.set(evt.value().to_string())
                }
            }

            //
            // FRONT BLOCKS
            //
            h2 { "Front Blocks" }
            for (i, block) in front_blocks.read().iter().cloned().enumerate() {
                BlockEditor {
                    block,
                    on_update: move |new_block| {
                        let mut blocks = front_blocks.write();
                        blocks[i] = new_block;
                    },
                    on_remove: move |_| {
                        let mut blocks = front_blocks.write();
                        blocks.remove(i);
                    }
                }
            }

            // ADDING BLOCKS
            button {
                onclick: move |_| {
                    front_blocks.write().push(Block::Text { value: "".into() });
                },
                "+ Add Text Block"
            }

            button {
                onclick: move |_| {
                    front_blocks.write().push(Block::Math { value: "".into() });
                },
                "+ Add Math Block"
            }

            button {
                onclick: move |_| {
                    // spawn async task because file picker is async
                    let mut front_blocks = front_blocks.clone();
                    spawn(async move {
                        // Call the plugin
                        let path = pick_image().await;
                        // Insert a new Block::Image into the editor
                        front_blocks.write().push(Block::Image { src: path });
                    });
                },
                "+ Add Image Block"
            }


            //
            // BACK BLOCKS
            //
            h2 { "Back Blocks" }
            for (i, block) in back_blocks.read().iter().cloned().enumerate() {
                BlockEditor {
                    block,
                    on_update: move |new_block| {
                        let mut blocks = back_blocks.write();
                        blocks[i] = new_block;
                    },
                    on_remove: move |_| {
                        let mut blocks = back_blocks.write();
                        blocks.remove(i);
                    }
                }
            }

            // adds an empty block
            button {
                onclick: move |_| {
                    back_blocks.write().push(Block::Text { value: "".into() });
                },
                "+ Add Back Block"
            }

            button {
                class: "save-btn",

                onclick: move |_| {
                    let name = card_name.read().clone();
                    let front = front_blocks.read().clone();
                    let back = back_blocks.read().clone();

                    spawn(async move {
                        match mode {
                            EditorMode::New { deck_id } => {
                                // 1. Create card in DB
                                let id = add_card(deck_id, name).await;

                                // 2. Save blocks
                                // because in api we call let js_args = serde_wasm_bindgen::to_value(&args).unwrap();
                                // we can pass by reference. Serde clones it and passes it to backend by value
                                save_card_blocks(id, &front, &back).await;
                            }

                            EditorMode::Edit { card_id } => {
                                // 1. Update the name (you need this backend function)
                                update_card_name(card_id, name).await;

                                // 2. Save updated blocks
                                save_card_blocks(card_id, &front, &back).await;
                            }
                        }

                        // Navigate back
                        nav.push(Route::CardListPage { id: c.deck_id });
                    });
                },

                "Save Card"
            }

        }
    }
}
