mod categorie;
mod players;
mod utils;
mod tableau_score;

use dioxus::prelude::*;

use crate::tableau_score::TableauScore;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const HYPERSCRIPT: Asset = asset!("/assets/_hyperscript.min.js");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Script { src: HYPERSCRIPT }
        TableauScore {}

    }
}

    
