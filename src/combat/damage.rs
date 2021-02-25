use crate::combat::{Attacker, Defender};
use crate::core::dice::{Advantage, DieSize, NumDice};
use bevy::ecs::{Bundle, Entity};
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
    num_dice: NumDice,
    die_size: DieSize,
    advantage: Advantage,
    flag_damage: FlatDamage,
    attacker: Attacker,
    defender: Defender,
    damage_type: DamageType,
}
