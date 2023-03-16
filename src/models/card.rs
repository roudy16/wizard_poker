#[derive(Debug, Clone, PartialEq)]
pub enum CardType {
    Creature,
    Instant,
    Sorcery,
    Enchantment,
    Artifact,
    Planeswalker,
    Land,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mana {
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Resource {
    Mana,
    Life,
    Card,
    Creature,
    Artifact,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceLocation {
    Hand,
    Library,
    Graveyard,
    Battlefield,
    Exile,
}

impl CardType {
    pub fn all() -> Vec<CardType> {
        vec![
            CardType::Creature,
            CardType::Instant,
            CardType::Sorcery,
            CardType::Enchantment,
            CardType::Artifact,
            CardType::Planeswalker,
            CardType::Land,
        ]
    }

    pub fn as_str(&self) -> &str {
        match self {
            CardType::Creature => "creature",
            CardType::Instant => "instant",
            CardType::Sorcery => "sorcery",
            CardType::Enchantment => "enchantment",
            CardType::Artifact => "artifact",
            CardType::Planeswalker => "planeswalker",
            CardType::Land => "land",
        }
    }
}

impl Mana {
    pub const fn all() -> [Mana; 6] {
        [
            Mana::White,
            Mana::Blue,
            Mana::Black,
            Mana::Red,
            Mana::Green,
            Mana::Colorless,
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CardBase {
    pub name: String,
    pub r#type: CardType,
    pub mana_cost: Vec<Mana>,
    pub components: Vec<CardComponent>,
}

impl CardBase {
    pub fn default() -> CardBase {
        CardBase {
            name: "Default".to_string(),
            r#type: CardType::Artifact,
            mana_cost: vec![Mana::Colorless],
            components: vec![],
        }
    }

    pub fn new(name: String, r#type: CardType, mana_cost: Vec<Mana>) -> CardBase {
        CardBase {
            name,
            r#type,
            mana_cost,
            components: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CardComponent {
    Creature(CreatureComponent),
    //Instant(CardInstant),
    //Sorcery(CardSorcery),
    //Enchantment(CardEnchantment),
    //Artifact(CardArtifact),
    //Planeswalker(CardPlaneswalker),
    //Land(CardLand),
}

trait Creature {
    fn power(&self) -> i32;
    fn toughness(&self) -> i32;
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreatureComponent {
    pub power: i32,
    pub toughness: i32,
}

impl Creature for CreatureComponent {
    fn power(&self) -> i32 {
        self.power
    }

    fn toughness(&self) -> i32 {
        self.toughness
    }
}
