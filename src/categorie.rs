
#[derive(Clone, Copy, PartialEq)]
pub enum Categories {
    Merveille,
    Guerre,
    Piece,
    Bleu,
    Jaune,
    Guilde,
    Science
}

#[derive(Clone, PartialEq)]
pub struct Categorie {
    pub couleur: &'static str,
    pub nom: &'static str,
    pub icon: &'static str,
    pub typ: Categories,
}

pub static CATEGORIES: &[Categorie] = &[
    Categorie {
        couleur: "#878a8cff",
        nom: "Merveilles",
        icon: "🏰",
        typ: Categories::Merveille,
    },
    Categorie {
        couleur: "#d43a3aff",
        nom: "La Guerre",
        icon: "⚔️",
        typ: Categories::Guerre,
    },
    Categorie {
        couleur: "#814e33ff",
        nom: "Pièces",
        icon: "🪙",
        typ: Categories::Piece,
    },
    Categorie {
        couleur: "#4b93dbff",
        nom: "Les Bleus",
        icon: "🏦",
        typ: Categories::Bleu,
    },
    Categorie {
        couleur: "#e1c33eff",
        nom: "Les jaunes",
        icon: "🤝",
        typ: Categories::Jaune,
    },
    Categorie {
        couleur: "#9f59e5ff",
        nom: "Guildes",
        icon: "⚛️",
        typ: Categories::Guilde,
    },
    Categorie {
        couleur: "#57c86aff",
        nom: "Science",
        icon: "🧩",
        typ: Categories::Science,
    },
];