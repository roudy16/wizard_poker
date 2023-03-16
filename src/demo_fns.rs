use crate::constants::ZERO_MANA;
use crate::models::card::{CardBase, CardType, Mana};
use crate::utils::create_basic_land;

pub fn create_one_of_each_card_type() {
    let card_types = CardType::all();
    let cards: Vec<CardBase> = card_types
        .iter()
        .map(|card_type| {
            CardBase::new(
                String::from("Test Card Name"),
                card_type.clone(),
                ZERO_MANA.clone(),
            )
        })
        .collect();
    println!("Cards: {:?}", cards);
}

pub fn create_one_of_each_basic_land() {
    let mana_types = Mana::all();
    let cards: Vec<CardBase> = mana_types
        .iter()
        .map(|mana_type| create_basic_land(mana_type.clone()))
        .collect();
    println!("Cards: {:?}", cards);
}
