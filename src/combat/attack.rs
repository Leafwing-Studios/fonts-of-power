use crate::combat::Active;
use crate::core::dice::Roll;
use bevy::ecs::{Commands, Entity, Query, With};
use derive_more::{Deref, DerefMut};

// TODO: add archetype invariants

/// Fundamental component for Attack entities, which store the information about an attack's effects
/// Attack entities should always have at least:
/// - Attack
/// - Attacker
/// - Defender
///
/// The following fields are added on later:
/// - AttackRoll
/// - Defense

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Attack;

#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq)]
pub struct Defense(i32);

/// Marker component to note that an attack landed
pub struct Landed;

pub fn check_attacks(
    attacks: Query<(Entity, &Roll, &Defense), With<(Active, Attack)>>,
    commands: &mut Commands,
) {
    for (attack_entity, attack_roll, defense) in attacks.iter() {
        if attack_roll.roll() >= **defense {
            commands.insert_one(attack_entity, Landed);
        }
    }
}
