use dioxus::prelude::*;
use shared::models::{ Card, Block, Deck, InsertBlockKind };
use crate::components::BlockEditor;
use crate::app::Route;
use crate::tauri_api::{
    get_card,
    add_card,
    update_card_metadata,
    save_card_blocks,
    pick_image,
    pick_archive,
};

async fn create_block(kind: InsertBlockKind) -> Option<Block> {
    match kind {
        InsertBlockKind::Text => Some(Block::Text { value: "".into() }),
        InsertBlockKind::Math => Some(Block::Math { value: "".into() }),
        InsertBlockKind::Image => {
            let path = pick_image().await;
            (!path.is_empty()).then(|| Block::Image { src: path })
        }
        InsertBlockKind::File => {
            let path = pick_archive().await;
            (!path.is_empty()).then(|| Block::File { path })
        }
    }
}


fn normalize_tags(raw: &str) -> Option<String> {
    let tags: Vec<String> = raw
        .split(',')
        .map(|t| t.trim().to_lowercase())
        .filter(|t| !t.is_empty())
        .collect();

    if tags.is_empty() {
        None
    } else {
        Some(tags.join(","))
    }
}



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
    let mut card_tags = use_signal(|| c.tags.clone().unwrap_or_default());
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
                    oninput: move |evt| card_name.set(evt.value())
                }
            }

            // Card tags
            div { class: "card-field",
                label { "Tags (comma separated)" }
                input {
                    placeholder: "e.g. ml, knn, classification",
                    value: "{card_tags}",
                    oninput: move |evt| card_tags.set(evt.value())
                }
            }


            //
            // FRONT BLOCKS
            //
            h2 { "Front Blocks" }

            for (i, block) in front_blocks.read().iter().cloned().enumerate() {
                BlockEditor {
                    block,

                    on_update: {
                        let mut front_blocks = front_blocks.clone();
                        move |new_block| {
                            front_blocks.write()[i] = new_block;
                        }
                    },

                    on_remove: {
                        let mut front_blocks = front_blocks.clone();
                        move |_| {
                            front_blocks.write().remove(i);
                        }
                    },

                    on_insert_above: {
                        let mut front_blocks = front_blocks.clone();
                        move |kind| {
                            let mut front_blocks = front_blocks.clone();
                            spawn(async move {
                                if let Some(block) = create_block(kind).await {
                                    front_blocks.write().insert(i, block);
                                }
                            });
                        }
                    },

                    on_insert_below: {
                        let front_blocks = front_blocks.clone();
                        move |kind| {
                            let mut front_blocks = front_blocks.clone();
                            spawn(async move {
                                if let Some(block) = create_block(kind).await {
                                    front_blocks.write().insert(i + 1, block);
                                }
                            });
                        }
                    },
                }
            }



            div { class: "add-block-buttons",
        
            // ADDING BLOCKS
            button {
                class: "button button-secondary",
                onclick: move |_| {
                    front_blocks.write().push(Block::Text { value: "".into() });
                },
                "+ Add Text Block"
            }

            button {
                class: "button button-secondary",
                onclick: move |_| {
                    front_blocks.write().push(Block::Math { value: "".into() });
                },
                "+ Add Math Block"
            }

            button {
                class: "button button-secondary",
                onclick: move |_| {
                    spawn(async move {
                        // Call the plugin
                        let path = pick_image().await;
                        if path != "" {
                            // Insert a new Block::Image into the editor
                            front_blocks.write().push(Block::Image { src: path });
                        }
                    });
                },
                "+ Add Image Block"
            }

            button {
                class: "button button-secondary",
                onclick: move |_| {
                        spawn(async move {
                        // Call the plugin
                        let path = pick_archive().await;
                        if path != "" {
                            front_blocks.write().push(Block::File { path: path });
                        }
                    });

                },
                "+ Add File Block"
            }

        }


            //
            // BACK BLOCKS
            //
            h2 { "Back Blocks" }

            for (i, block) in back_blocks.read().iter().cloned().enumerate() {
                BlockEditor {
                    block,

                    on_update: {
                        let mut back_blocks = back_blocks.clone();
                        move |new_block| {
                            back_blocks.write()[i] = new_block;
                        }
                    },

                    on_remove: {
                        let mut back_blocks = back_blocks.clone();
                        move |_| {
                            back_blocks.write().remove(i);
                        }
                    },

                    on_insert_above: {
                        let mut back_blocks = back_blocks.clone();
                        move |kind| {
                            let mut back_blocks = back_blocks.clone();
                            spawn(async move {
                                if let Some(block) = create_block(kind).await {
                                    back_blocks.write().insert(i, block);
                                }
                            });
                        }
                    },

                    on_insert_below: {
                        let back_blocks = back_blocks.clone();
                        move |kind| {
                            let mut back_blocks = back_blocks.clone();
                            spawn(async move {
                                if let Some(block) = create_block(kind).await {
                                    back_blocks.write().insert(i + 1, block);
                                }
                            });
                        }
                    },
                }
            }


            div { class: "add-block-buttons",
            // ADDING BLOCKS
            button {
                class: "button button-secondary",
                onclick: move |_| {
                    back_blocks.write().push(Block::Text { value: "".into() });
                },
                "+ Add Text Block"
            }

            button {
                class: "button button-secondary",
                onclick: move |_| {
                    back_blocks.write().push(Block::Math { value: "".into() });
                },
                "+ Add Math Block"
            }

            button {
                class: "button button-secondary",
                onclick: move |_| {
                    spawn(async move {
                        let path = pick_image().await;
                        if path != "" {
                            // Insert a new Block::Image into the editor
                            back_blocks.write().push(Block::Image { src: path });
                        }
                    });
                },
                "+ Add Image Block"
            }

            button {
                class: "button button-secondary",
                onclick: move |_| {
                        spawn(async move {
                        // Call the plugin
                        let path = pick_archive().await;
                        web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!(
                            "path returned from file picker is: {}",
                            path
                        )));
                        if path != "" {
                            back_blocks.write().push(Block::File { path: path });
                        }
                    });

                },
                "+ Add File Block"
            }
        }

            button {
                class: "button button-primary save-button",

                onclick: move |_| {
                    let name = card_name.read().clone();
                    let tags = normalize_tags(&card_tags.read());
                    let front = front_blocks.read().clone();
                    let back = back_blocks.read().clone();

                    spawn(async move {
                        match mode {
                            EditorMode::New { deck_id } => {
                                // 1️⃣ Create card
                                let id = add_card(deck_id, name.clone()).await;

                                // 2️⃣ Update metadata (name + tags)
                                update_card_metadata(id, name, tags).await;

                                // 3️⃣ Save blocks
                                save_card_blocks(id, &front, &back).await;
                            }

                            EditorMode::Edit { card_id } => {
                                // 1️⃣ Update metadata
                                update_card_metadata(card_id, name, tags).await;

                                // 2️⃣ Save blocks
                                save_card_blocks(card_id, &front, &back).await;
                            }
                        }

                        nav.push(Route::CardListPage { id: c.deck_id });
                    });
                },

                "Save Card"
            }

        }
    }
}
