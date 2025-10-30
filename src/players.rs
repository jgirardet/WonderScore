use crate::{categorie::Categories};


#[derive(Clone)]
pub struct Player {
    pub id: i32,
    pub nom: String,
    pub guerre: i32,
    pub bleu: i32,
    pub science: i32,
    pub guilde: i32,
    pub jaune: i32,
    pub piece: i32,
    pub merveille: i32,
}

impl Player {
    pub fn new(name: &str, id: i32) -> Player {
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

    pub fn value(&self, cate: Categories) -> i32 {
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

    pub fn set(&mut self, cate: Categories, value: i32) {
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
pub struct Players {
    pub(crate) value: Vec<Player>,
}

impl Players {

    pub fn new() -> Players {
        Players {
        value: vec![
            Player::new("Papa", 1),
            Player::new("Maman", 2),
            Player::new("Axel1234567", 3),
            Player::new("Margaux", 4),
            Player::new("Margaux5", 5),
            // Player::new("Margaux6", 6),
            // Player::new("Margaux7", 7),
        ],
    }
    }
    pub fn totaux(&self) -> Totaux {
        self.value.iter().map(|p| p.total()).collect()
        
    }

    pub fn update(&mut self, user_id: i32, cate: Categories, value: i32) {
        for p in self.value.iter_mut() {
            if p.id == user_id {
                p.set(cate, value);
                return;
            }
        }
    }

    pub fn len(&self) -> usize {
        return self.value.len();
    }

    pub fn players(&self) -> Vec<Player> {
        self.value.clone()
    }
}

pub type Totaux = Vec<i32>;
