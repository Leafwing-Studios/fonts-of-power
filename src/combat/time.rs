use bevy::prelude::{Component, Entity};

#[non_exhaustive]
#[derive(Component, Copy, Clone, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Duration {
    EndOfTurn,
    StartOfYourNextTurn,
    EndOfYourNextTurn,
    Concentration,
    Rounds(usize),
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Turn;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Round;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TurnOrder {
    entity_order: Vec<Entity>,
}
