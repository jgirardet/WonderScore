use dioxus::prelude::*;

use crate::players::Players;

#[component]
pub fn ResetButton(players: Signal<Players>) -> Element {
    rsx! {
     
            button {
                class: "px-4 py-2 bg-red-200 text-green font-semibold rounded-lg shadow-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-offset-2 transition",
                onclick: move |_| {
                    players.set(players().reset());
                },
                "Reset",
            }
        
    }
}