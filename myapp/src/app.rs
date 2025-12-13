use dioxus::prelude::*;
use dioxus::document::{Script, Stylesheet};
use dioxus_router::prelude::*;
use crate::components::{ DeckList, CardView, CardListPage, CardEditorEdit, CardEditorNew };
use shared::models::*;

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    DeckList,

    #[route("/deck/:id")] 
    CardListPage {id: i64,},

    #[route("/card/:id")] 
    CardView { id: i64, },

    #[route("/card/:id/edit")]
    CardEditorEdit { id: i64 },

    #[route("/card/new/:deck_id")]
    CardEditorNew { deck_id: i64 },
}

static CSS: Asset = asset!("/assets/styles.css");


#[component]
pub fn App() -> Element {
    rsx! {
        // Load MathJax config FIRST
        Script {
            src: asset!("./assets/mathjax-config.js"),
        },

        // Local MathJax core (NO CDN)
        Script {
            src: asset!("./assets/mathjax/es5/tex-mml-chtml.js"),
        }
/*         // Load MathJax core
        Script {
            src: "https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-mml-chtml.js",
        }, */
        Stylesheet { href: CSS },
        Router::<Route> {}
    }
}
