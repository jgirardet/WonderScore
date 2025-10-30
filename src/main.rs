use dioxus::{document::eval, prelude::*};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const BOOTSTRAP_CSS: Asset = asset!("/assets/bootstrap.css");
const AUTRE_CSS: Asset = asset!("/assets/autre.css");
const HYPERSCRIPT: Asset = asset!("/assets/_hyperscript.min.js");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        document::Link { rel: "stylesheet", href: AUTRE_CSS }
        document::Script{src: HYPERSCRIPT}
        Hero {}

    }
}
#[derive(Clone, Copy, PartialEq)]
enum Categories {
    Merveille,
    Guerre,
    Piece,
    Bleu,
    Jaune,
    Guilde,
    Science,
}

#[derive(Clone, PartialEq)]
struct Categorie {
    couleur: &'static str,
    nom: &'static str,
    typ: Categories,
}

static CATEGORIES: &[Categorie] = &[
    Categorie {
        couleur: "#878a8cff",
        nom: "🏰 Merveilles",
        typ: Categories::Merveille,
    },
    Categorie {
        couleur: "#d43a3aff",
        nom: "⚔️ La Guerre",
        typ: Categories::Guerre,
    },
    Categorie {
        couleur: "#814e33ff",
        nom: "🪙 Pièces",
        typ: Categories::Piece,
    },
    Categorie {
        couleur: "#4b93dbff",
        nom: "🏦 Les Bleus",
        typ: Categories::Bleu,
    },
    Categorie {
        couleur: "#e1c33eff",
        nom: "🤝 Les jaunes",
        typ: Categories::Jaune,
    },
    Categorie {
        couleur: "#9f59e5ff",
        nom: "⚛️ Guildes",
        typ: Categories::Guilde,
    },
    Categorie {
        couleur: "#57c86aff",
        nom: "🧩 Science",
        typ: Categories::Science,
    },
];

#[derive(Clone)]
struct Player {
    id: i32,
    nom: String,
    guerre: i32,
    bleu: i32,
    science: i32,
    guilde: i32,
    jaune: i32,
    piece: i32,
    merveille: i32,
}

impl Player {
    fn new(name: &str, id: i32) -> Player {
        Player {
            id,
            nom: name.to_string(),
            guerre: 0,
            bleu: 0,
            science: 0,
            guilde: 0,
            jaune: 0,
            piece: 0,
            merveille: 0,
        }
    }

    fn value(&self, cate: Categories) -> i32 {
        match cate {
            Categories::Bleu => self.bleu,
            Categories::Guerre => self.guerre,
            Categories::Merveille => self.merveille,
            Categories::Guilde => self.guilde,
            Categories::Jaune => self.jaune,
            Categories::Piece => self.piece,
            Categories::Science => self.science,
        }
    }

    fn set(&mut self, cate: Categories, value: i32) {
        match cate {
            Categories::Bleu => self.bleu = value,
            Categories::Guerre => self.guerre = value,
            Categories::Merveille => self.merveille = value,
            Categories::Guilde => self.guilde += value,
            Categories::Jaune => self.jaune = value,
            Categories::Piece => self.piece = value,
            Categories::Science => self.science = value,
        };
    }

    fn total(&self) -> i32 {
        self.guerre
            + self.guilde
            + self.bleu
            + self.jaune
            + self.merveille
            + self.piece
            + self.science
    }
}

#[derive(Clone)]
struct Players {
    value: Vec<Player>,
}

impl Players {
    fn totaux(&self) -> Totaux {
        Totaux {
            value: self.value.iter().map(|p| p.total()).collect(),
        }
    }

    fn update(&mut self, user_id: i32, cate: Categories, value: i32) {
        for p in self.value.iter_mut() {
            if p.id == user_id {
                p.set(cate, value);
                return;
            }
        }
    }

    fn players(&self) -> Vec<Player> {
        self.value.clone()
    }
}

#[derive(Props, Clone, PartialEq)]
struct Totaux {
    value: Vec<i32>,
}

#[component]
pub fn Hero() -> Element {
    let mut players = use_signal(|| Players {
        value: vec![
            Player::new("Papa", 1),
            Player::new("Maman", 2),
            Player::new("Axel", 3),
            Player::new("Margaux", 4),
            // Player::new("Margaux5", 5),
            // Player::new("Margaux6", 6),
            // Player::new("Margaux7", 7),
        ],
    });

    let mut update = move |user_id: i32, cate: Categories, e: Event<FormData>| {
        if let Ok(val) = e.value().parse() {
            players.write().update(user_id, cate, val)
        };
    };

    rsx! {
        div { class: "container-fluid",
            background_color: "#ffffffff",
            div { id: "main", class: "row",
                div {class: "col-3"}
                div {
                    class: "col",
                    div {
                        class: "row",
                        for user in players.read().players() {
                    div {
                        id: "{user.id}",
                        class: "col text-center text-truncate",
                        span { "{user.nom}" }
                    }
                }
                    }

                }

            }
            for categorie in CATEGORIES.iter() {
                div { class: "row my-2 ",
                    div {
                        class: "col-3 text-white text-nowrap",
                        background_color: "{categorie.couleur}",
                        // "{categorie.nom}"
                        span { class: "align-middle", "{categorie.nom}" }
                    }
                    div { class: "col",
                        div { class: "row",
                            for user in players().value {
                                TabCell {
                                    categorie: categorie.clone(),
                                    class: "text-center col",
                                    InputScore {
                                        score: user.value(categorie.typ),
                                        oninput: move |e| update(user.id, categorie.typ, e),
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Total { val: players().totaux() }
            button {
                class: "btn btn-outline-danger",
                
                onclick:  move |_| {
                    players.write().value.clear();
            players.write().value.push(Player::new("Papa", 1));
            players.write().value.push(Player::new("Maman", 2));
            players.write().value.push(Player::new("Axel", 3));
            players.write().value.push(Player::new("Margaux", 4));
                },
                "Reset"
            // Player::new("Maman", 2),
            // Player::new("Axel", 3),
            // Player::new("Margaux", 4),
            // Player::new("Margaux5", 5),
            // Player::new("Margaux6", 6),
            // Player::new("Margaux7", 7),


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
            class: format!("text-white {}", class.unwrap_or_default()),
            background_color: "{categorie.couleur}",
            {children}
        }
    )
}

#[component]
fn InputScore(score: i32, oninput: EventHandler<FormEvent>) -> Element {
    let mut show_popup = use_signal(|| false);
    let mut selected_number = use_signal(|| 0);
    rsx! {
        input {
            r#type: "number",
            value: score, //user.value(categorie.typ),
            class: "form-control p-0 m-0 bg-transparent text-white",
            text_align: "center",
            readonly: false,
            inputmode: "numeric",
            oninput,
            "_": "on focus call me.select()",
            // onfocus: move|e: Event<FocusData>| {
            //     e.
            //     eval("this.select()");}
            //onclick: move|_| {show_popup.toggle();},
            //  NumberPickerPopup {
            //     show: show_popup,
            //     min: 1,
            //     max: 20,
            //     on_select: move |num| {
            //         selected_number.set(num);
            //     }
            // }
        }
    }
}

#[component]
fn Total(val: Totaux) -> Element {
    rsx!(
        div { class: "row",
            div { class: "col-3 text-nowrap", "🟰 " }
            div { class: "col",
                div { class: "row",
                    for tot in val.value {
                        div { class: "col text-center", "{tot}" }
                    }
                }
            }
        }
    )
}

// #[component]
// fn Pickup() -> Element {
//     class: ""
// }
