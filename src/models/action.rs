#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    PlayCard,
    CastCard,
    ActivateAbility,
    Attack,
    Block,
    Tap,
    Untap,
    Discard,
    Draw,
    Shuffle,
    Exile,
    Sacrifice,
    Destroy,
    GainResource,
    LoseResource,
}
