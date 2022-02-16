use bevy::prelude::{Component, Entity};
use derive_more::{Deref, DerefMut};

#[non_exhaustive]
#[derive(Component, Copy, Clone, Hash, PartialEq, Eq)]
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

#[derive(Clone, Hash, PartialEq, Eq, Deref, DerefMut)]
pub struct TurnOrder(Vec<Entity>);
