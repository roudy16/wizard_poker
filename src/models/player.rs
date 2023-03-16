use crate::models::card::CardBase;

pub struct Player {
    pub name: String,
    pub life: i32,
    pub hand: Vec<CardBase>,
    pub library: Vec<CardBase>,
    pub graveyard: Vec<CardBase>,
}

impl Player {
    pub fn new(name: String, life: i32) -> Player {
        Player {
            name,
            life,
            hand: Vec::new(),
            library: Vec::new(),
            graveyard: Vec::new(),
        }
    }
}
