mod categorie;
mod manage_players;
mod players;
mod reset;
mod tableau_score;
mod utils;

use dioxus::prelude::*;

use crate::{
    manage_players::ManagePlayers, players::Players, reset::ResetButton,
    tableau_score::TableauScore,
};
const FAVICON: Asset = asset!("/assets/favicon.ico");
const HYPERSCRIPT: Asset = asset!("/assets/_hyperscript.min.js");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let nb_joueurs = use_signal(|| 4u8);
    let mut players = use_signal(|| Players::new(4));
    use_effect(move || {
        players.set(Players::new(nb_joueurs() as usize));
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Script { src: HYPERSCRIPT }
        div {

            class: "px-2 py-3",
            div { 
                class: "flex justify-around",
                ManagePlayers { nb_joueurs }
                ResetButton { players }
            }
            TableauScore { players }
        }

    }
}
