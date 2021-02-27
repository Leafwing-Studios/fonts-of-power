use crate::combat::Active;
use crate::core::dice::Roll;
use bevy::ecs::{Commands, Entity, Query, With};
use derive_more::{Deref, DerefMut};
use num_rational::Ratio;

// TODO: add archetype invariants

/// Fundamental component for Attack entities, which store the information about an attack's effects
/// Attack entities should always have at least:
/// - Attack
/// - Attacker
/// - Defender
/// - AttackRoll
/// - Defense
/// - CritThreshold
/// - Efficacy
///
/// The following optional fields are commonly added:
/// - Damage
/// - DamageRoll
/// - DamageType
/// - Afflictions
/// - Ailments
/// - ForcedMovement
/// - various components with the Status field

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Attack;

#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq)]
pub struct Defense(i32);
#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq)]
pub struct CritThreshold(u8);
#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq)]
pub struct Efficacy(Ratio<u16>);

/// Rolls the attack
pub fn roll_attacks() {}

/// Attacks are cloned to each of the targets
pub fn dispatch_attacks() {}

/// Fetches fresh statistics for the attacker and defender
pub fn prepare_attacks() {}

/// Marker component to note that an attack landed
pub struct Landed;

/// Marker component to note that an attack was a critical hit
pub struct Crit;

/// Each attack is checked to see if it landed
pub fn check_attacks(
    attacks: Query<(Entity, &Roll, &Defense, &CritThreshold), With<(Active, Attack)>>,
    commands: &mut Commands,
) {
    for (attacking_entity, attack_roll, defense, crit_threshold) in attacks.iter() {
        let final_attack = attack_roll.roll();
        let natural_roll = final_attack - attack_roll.modifier;

        if final_attack >= **defense {
            commands.insert_one(attacking_entity, Landed);

            if natural_roll as u8 >= **crit_threshold {
                commands.insert_one(attacking_entity, Crit);
            }
        }
    }
}

/// The efficacy of attacks which were a critical hit are doubled
pub fn apply_crits(mut query: Query<&mut Efficacy, With<(Active, Crit, Attack)>>) {
    for mut efficacy in query.iter_mut() {
        **efficacy *= 2;
    }
}
