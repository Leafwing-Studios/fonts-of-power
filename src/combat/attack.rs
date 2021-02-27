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
/// - CritThreshold
/// - Efficacy

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Attack;

#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq)]
pub struct Defense(i32);
#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq)]
pub struct CritThreshold(u8);
#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq)]
pub struct Efficacy(i32);

/// Marker component to note that an attack landed
pub struct Landed;

/// Marker component to note that an attack was a critical hit
pub struct Crit;

pub fn check_attacks(
    attacks: Query<(Entity, &Roll, &Defense, &CritThreshold), With<(Active, Attack)>>,
    commands: &mut Commands,
) {
    for (attack_entity, attack_roll, defense, crit_threshold) in attacks.iter() {
        let final_attack = attack_roll.roll();
        let natural_roll = final_attack - attack_roll.modifier;

        if final_attack >= **defense {
            commands.insert_one(attack_entity, Landed);

            if natural_roll as u8 >= **crit_threshold {
                commands.insert_one(attack_entity, Crit);
            }
        }
    }
}

// TODO: add crit processing system

// TODO: add AttackEffect dispatching system

// TODO: add archetype invariants
/// Fundamental marker component for AttackEffect entities
/// Entities that have an AttackEffect component should always also have:
/// - AttackEffect
/// - Attacker
/// - Defender
///
/// Eventually, Damage, Afflictions, Conditions and various Status components may be added to them as well
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AttackEffect;
