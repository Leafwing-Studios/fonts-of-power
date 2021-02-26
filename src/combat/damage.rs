use crate::combat::{Attacker, Defender};
use crate::core::dice::Roll;
use bevy::ecs::Bundle;
use derive_more::{Deref, DerefMut};
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DamageType {
    Pure(Element),
    Hybrid(Element, Element),
    Split(Element, Element),
}
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct FlatDamage(i32);

/// Fundamental component for Damage entities, which store the information about damage that is about to be dealt
/// Damage entities should always have at least:
/// - Damage
/// - NumDice
/// - DieSize
/// - Advantage
/// - FlatDamage
/// - Attacker
/// - Defender
/// - DamageType
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Damage;

#[derive(Bundle)]
pub struct DamageBundle {
    damage: Damage,
    attacker: Attacker,
    defender: Defender,
    roll: Roll,
    damage_type: DamageType,
}
