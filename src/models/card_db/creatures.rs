use crate::models::card::{CardBase, CardComponent, CardType, CreatureComponent, Mana};

pub fn create_bear() -> CardBase {
    CardBase {
        name: "Bear".to_string(),
        r#type: CardType::Creature,
        mana_cost: vec![Mana::Colorless, Mana::Green],
        components: vec![CardComponent::Creature(CreatureComponent {
            power: 2,
            toughness: 2,
        })],
    }
}
