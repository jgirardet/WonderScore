use dioxus::prelude::*;

#[component]
pub fn ManagePlayers(nb_joueurs: Signal<u8>) -> Element {
    rsx!(



        select {
            id: "cars",
            name: "cars",
            class: "px-4 py-2 bg-red-200 text-green font-semibold rounded-lg shadow-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-400 focus:ring-offset-2 transition",
            value: nb_joueurs(),
            oninput: move |e: Event<FormData>| nb_joueurs.set(e.value().parse::<u8>().unwrap()), //players.write().one(),
            option { value: 2, selected: selected(2, nb_joueurs()), "2 joueurs" }
            option { value: 3, selected: selected(3, nb_joueurs()), "3 joueurs" }
            option { value: 4, selected: selected(4, nb_joueurs()), "4 joueurs" }
            option { value: 5, selected: selected(5, nb_joueurs()), "5 joueurs" }
            option { value: 6, selected: selected(6, nb_joueurs()), "6 joueurs" }
            option { value: 7, selected: selected(7, nb_joueurs()), "7 joueurs" }
        }

    )
}

fn selected(val: u8, actuel: u8) -> bool {
    val == actuel
}
