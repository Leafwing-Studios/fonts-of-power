use crate::core::dice::{Advantage, DieSize};
use bevy::ecs::Entity;
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Element {
    Physical,
    Air,
    Earth,
    Fire,
    Water,
    Eldritch,
    Arcane,
    Radiant,
    Umbral,
    Primal,
    Decay,
    Electric,
    Corrosive,
}
#[allow(dead_code)]
pub enum DamageType {
    Pure(Element),
    Hybrid(Element, Element),
    Split(Element, Element),
}

#[allow(dead_code)]
/// Emit multiple damage events for complex damage calculations
pub struct DamageEvent {
    n: i8,
    d: DieSize,
    advantage: Advantage,
    flat: i32,
    damage_type: Vec<DamageType>,
    attacker: Entity,
    defender: Entity,
}
