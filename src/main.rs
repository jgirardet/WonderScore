mod categorie;
mod players;
mod utils;

use dioxus::prelude::*;

use crate::categorie::{Categorie, CATEGORIES};
use crate::players::{Player, Players, Totaux};
use crate::utils::initial_width;

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

#[component]
pub fn TableauScore() -> Element {
    let width = initial_width();

    let mut players = use_signal(|| Players::new());
    rsx! {
        div {
            class: "text-md sm:text-lg font-bold text-white px-2",
            background_color: "#ffffffff",
            LigneJoueurs {players}
            for categorie in CATEGORIES.iter() {
                LigneScore {
                    width: width(),
                    categorie: categorie.clone(),
                    players,
                }
            }
            LigneTotal { totaux: players().totaux() }
            button {
                class: "btn btn-outline-danger",
                onclick: move |_| {
                    players.write().value.clear();
                    players.write().value.push(Player::new("Papa", 1));
                    players.write().value.push(Player::new("Maman", 2));
                    players.write().value.push(Player::new("Axel", 3));
                    players.write().value.push(Player::new("Margaux", 4));
                },
                "Reset"
            }
        }
    }
}

#[component]
fn TabCell(
    categorie: Categorie,
    #[props(into)] class: Option<String>,
    children: Element,
) -> Element {
    rsx!(
        div {
            class: format!("flex justify-center {}", class.unwrap_or_default()),
            background_color: "{categorie.couleur}",
            {children}
        }
    )
}


#[component]
fn LigneJoueurs(players: ReadSignal<Players>)-> Element {
    rsx!(
        div { id: "main", class: "flex h-20 gap-2 text-black items-center",
            Col0  {nb_players:players().len(),class:"opacity-0" ,"7️⃣"} // keep for alignment
            for user in players.read().players() {
                div { id: "{user.id}", class: "flex-1 min-w-0 text-center",
                    div { class: "truncate", "{user.nom}" }
                }
            }
        }
    )
}

#[component]
fn LigneScore(width: u32, categorie: Categorie, players: Signal<Players>) -> Element {

    rsx!(
        div { class: "flex h-20", background_color: "{categorie.couleur}",

            Col0 {nb_players:players().len(), class: "flex text-center gap-1 sm:gap-3 items-center justify-center",
                p { class: "", title: "{categorie.nom}", "{categorie.icon}" }
                if players().len() > 4 && width >= 640 {
                    p { "{categorie.nom}" }
                }
            }
            for user in players().value {
                TabCell { categorie: categorie.clone(), class: "flex-1",
                    InputScore {
                        score: user.value(categorie.typ),
                        oninput: move |e: Event<FormData>| {
                            if let Ok(val) = e.value().parse() {
                                players.write().update(user.id, categorie.typ, val)
                            }
                        },
                    
                    }
                }
            }
        }
    )
}



#[component]
fn InputScore(score: i32, oninput: EventHandler<FormEvent>) -> Element {
    rsx! {
        input {
            r#type: "number",
            value: score, //user.value(categorie.typ),
            class: "w-8  [appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none",
            text_align: "center",
            readonly: false,
            inputmode: "numeric",
            oninput,
            "_": "on focus call me.select()",
        }
    }
}

#[component]
fn LigneTotal(totaux: Totaux) -> Element {
    rsx!(
        div { class: "flex h-20 items-center text-center text-black",
            Col0 {nb_players:totaux.len(), "🟰" }
            for tot in totaux {
                div { class: "flex-1", "{tot}" }
            }
        }
    )
}



#[component]
fn Col0(nb_players:usize, class:Option<String>,  children:Element) -> Element {
        let mut width = "w-30 sm:w-50";
        if nb_players > 4 {
        width =  "flex-shrink";
        }
        let c = class.unwrap_or_default();
        rsx!(
            div { class: "{width} sm:w-50 px-1 {c}", {children} }
        )
    
}

    
