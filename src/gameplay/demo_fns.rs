use rand;

use crate::models::card::{CardBase, Mana};
use crate::models::card_db::creatures::create_bear;
use crate::utils::create_basic_land;

static ALL_MANA: [Mana; 6] = Mana::all();

pub fn create_random_deck() -> Vec<CardBase> {
    let deck_size: usize = 60;
    let mut deck = Vec::with_capacity(deck_size);
    for _ in 0..deck_size {
        let rand_num = rand::random::<usize>();
        if (rand_num % 2) == 0 {
            let rand_mana_idx = rand::random::<usize>() % ALL_MANA.len();
            let random_mana = ALL_MANA[rand_mana_idx].clone();
            deck.push(create_basic_land(random_mana));
        } else {
            deck.push(create_bear());
        }
    }

    deck
}
