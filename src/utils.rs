use crate::constants::{
    FOREST_NAME, ISLAND_NAME, MOUNTAIN_NAME, PLAINS_NAME, SWAMP_NAME, WASTES_NAME, ZERO_MANA,
};
use crate::models::card::{CardBase, CardType, Mana};

pub fn create_basic_land(mana_type: Mana) -> CardBase {
    CardBase::new(
        match mana_type {
            Mana::White => PLAINS_NAME,
            Mana::Blue => ISLAND_NAME,
            Mana::Black => SWAMP_NAME,
            Mana::Red => MOUNTAIN_NAME,
            Mana::Green => FOREST_NAME,
            Mana::Colorless => WASTES_NAME,
        }
        .to_string(),
        CardType::Land,
        ZERO_MANA.clone(),
    )
}
