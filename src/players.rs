use derive_more::IntoIterator;

use crate::{categorie::Categories};


#[derive(Clone, PartialEq)]
pub struct Player {
    pub id: usize,
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
    pub fn new(name: &str, id: usize) -> Player {
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

#[derive(Clone, IntoIterator, PartialEq)]
pub struct Players(pub Vec<Player>);

impl Players {

    pub fn new(nb: usize) -> Players {
        let mut noms = vec!["Papa", "Maman", "Axel", "Margaux", "Joueur5", "Joueur6", "Joueur7"];
        noms.truncate(nb);
        Players(noms.into_iter().enumerate().map(|(idx, p)| Player::new(p, idx)).collect())
    
    }

    pub fn reset(&self) -> Players {
        Players::new(self.len())
    }

    pub fn totaux(&self) -> Totaux {
        self.0.iter().map(|p| p.total()).collect()
        
    }

    pub fn update(&mut self, user_id: usize, cate: Categories, value: i32) {
        for p in self.0.iter_mut() {
            if p.id == user_id {
                p.set(cate, value);
                return;
            }
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    
}

pub type Totaux = Vec<i32>;
