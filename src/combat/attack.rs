use crate::combat::Active;
use crate::core::dice::{roll, Advantage, DieSize, NumDice};
use bevy::ecs::{Commands, Entity, Query, With};
use derive_more::{Deref, DerefMut};

// TODO: add archetype invariants

/// Fundamental component for Attack entities, which store the information about an attack's effects
/// Attack entities should always have at least:
/// - Attack
/// - Attacker
/// - Defender
/// - Advantage
///
/// The following fields are added on later:
/// - AttackBonus
/// - Defense
/// - Advantage
#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Attack;

#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq)]
pub struct AttackBonus(i8);
#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq)]
pub struct Defense(i8);

/// Marker component to note that an attack landed
pub struct Landed;

pub fn check_attacks(
    attacks: Query<(Entity, &AttackBonus, &Defense, &Advantage), With<(Active, Attack)>>,
    commands: &mut Commands,
) {
    for (attack_entity, attack_bonus, defense, advantage) in attacks.iter() {
        if roll(NumDice(1), DieSize::d20, *advantage) as i8 + **attack_bonus >= **defense {
            commands.insert_one(attack_entity, Landed);
        }
    }
}
