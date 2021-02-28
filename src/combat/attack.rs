use crate::combat::actions::Targets;
use crate::combat::Active;
use crate::core::dice::Roll;
use crate::core::entity_clone::*;
use crate::core::stats::Attribute;
use bevy::ecs::{Commands, Entity, Query, With};
use derive_more::{Deref, DerefMut};
use num_rational::Ratio;

// TODO: add archetype invariants

/// Fundamental component for Attack entities, which store the information about an attack's effects
/// Attack entities should always have the following fields, typically derived from the action taken by the attacker:
/// - Attack
/// - Attacker
/// - Targets
/// - AttackType
/// - AttackRoll
/// - CritThreshold
/// - Efficacy
/// - TargetArity
/// - RangeCategory
///
/// The following fields are optional:
/// - AreaOfEffect
///
/// The following optional fields are commonly added during attack processing:
/// - Damage
/// - DamageRoll
/// - DamageType
/// - Afflictions
/// - Ailments
/// - ForcedMovement
/// - various components with the Status field
///
/// The following field is added during attack dispatch, when the attack entities are cloned:
/// - Defender
/// - Defense

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Attack;
#[derive(Clone, Debug, Deref, DerefMut, PartialEq, Eq)]
pub struct Attacker(Entity);
#[derive(Clone, Debug, Deref, DerefMut, PartialEq, Eq)]
pub struct Defender(Entity);
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttackRoll {
    roll: Roll,
    result: Option<i32>,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AttackType {
    Basic,
    Special(Attribute),
    NoRoll,
}

#[derive(Copy, Clone, Debug)]
pub struct AttackBonus {
    basic: i32,
    special: i32,
}

impl AttackBonus {
    pub fn get(self, attack_type: AttackType) -> Option<i32> {
        match attack_type {
            AttackType::Basic => Some(self.basic),
            AttackType::Special(_) => Some(self.special),
            AttackType::NoRoll => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Defenses {
    basic: i32,
    prowess: i32,
    agility: i32,
    expertise: i32,
    focus: i32,
    presence: i32,
}

impl Defenses {
    pub fn get(self, attack_type: AttackType) -> Option<i32> {
        match attack_type {
            AttackType::Basic => Some(self.basic),
            AttackType::Special(attr) => Some(self.special_defense(attr)),
            AttackType::NoRoll => None,
        }
    }

    pub fn special_defense(self, attr: Attribute) -> i32 {
        match attr {
            Attribute::Prowess => self.prowess,
            Attribute::Agility => self.agility,
            Attribute::Expertise => self.expertise,
            Attribute::Focus => self.focus,
            Attribute::Presence => self.presence,
        }
    }
}

#[derive(Clone, Debug, Deref, DerefMut, PartialEq, Eq)]
pub struct Defense(Option<i32>);
#[derive(Clone, Debug, Deref, DerefMut, PartialEq, Eq)]
pub struct CritThreshold(u8);
#[derive(Clone, Debug, Deref, DerefMut, PartialEq, Eq)]
pub struct Efficacy(Ratio<u16>);

/// Gets the attack bonus for the roll
pub fn get_attack_bonuses(
    mut attack_query: Query<(&mut AttackRoll, &Attacker, &AttackType), With<Active>>,
    attacker_query: Query<&AttackBonus>,
) {
    for (mut attack_roll, attacker, attack_type) in attack_query.iter_mut() {
        let attack_bonuses = attacker_query.get(**attacker).unwrap();
        let attack_bonus = attack_bonuses.get(*attack_type);

        if let Some(bonus) = attack_bonus {
            attack_roll.roll.modifier = bonus
        }
    }
}

/// Rolls the attack
pub fn roll_attacks(mut query: Query<&mut AttackRoll, With<Active>>) {
    for mut i in query.iter_mut() {
        i.result = Some(i.roll.roll());
    }
}

/// Attacks are cloned to each of the targets
pub fn dispatch_attacks(
    query: Query<(Entity, &Targets), With<(Attack, Active)>>,
    commands: &mut Commands,
) {
    for (attack_entity, targets) in query.iter() {
        for target in targets.iter() {
            commands.clone_effect(attack_entity, *target);
        }
    }
}

/// Gets the appropriate defense value for each defender
pub fn get_defenses(
    mut attack_query: Query<(&mut Defense, &Defender, &AttackType), With<Active>>,
    defender_query: Query<&Defenses>,
) {
    for (mut defense, defender, attack_type) in attack_query.iter_mut() {
        let defenses = defender_query.get(**defender).unwrap();

        **defense = defenses.get(*attack_type);
    }
}

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

        // TODO: handle case where no attack roll is needed
        if final_attack >= defense.unwrap() {
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
