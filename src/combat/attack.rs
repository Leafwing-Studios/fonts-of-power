use crate::combat::actions::Targets;
use crate::combat::conditions::{Afflictions, Ailments};
use crate::combat::damage::DamageRoll;
use crate::combat::forced_movement::ForcedMovement;
use crate::combat::Active;
use crate::core::dice::Roll;
use crate::core::stats::{Absorption, Attribute};
use bevy::prelude::Component;
use bevy::prelude::{Commands, Entity, Query, With};
use num_rational::Ratio;
use std::ops::Mul;

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

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Attack;

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Attacker {
    pub entity: Entity,
}

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Defender {
    pub entity: Entity,
}

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct AttackRoll {
    modifier: isize,
    roll: Roll,
}
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq)]
pub enum AttackType {
    Basic,
    Special(Attribute),
    NoRoll,
}

#[derive(Component, Copy, Clone, Debug)]
pub struct AttackBonus {
    basic: isize,
    special: isize,
}

impl AttackBonus {
    pub fn get(self, attack_type: AttackType) -> Option<isize> {
        match attack_type {
            AttackType::Basic => Some(self.basic),
            AttackType::Special(_) => Some(self.special),
            AttackType::NoRoll => None,
        }
    }
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Defenses {
    basic: usize,
    prowess: usize,
    agility: usize,
    expertise: usize,
    focus: usize,
    presence: usize,
}

impl Defenses {
    pub fn get(self, attack_type: AttackType) -> Option<usize> {
        match attack_type {
            AttackType::Basic => Some(self.basic),
            AttackType::Special(attr) => Some(self.special_defense(attr)),
            AttackType::NoRoll => None,
        }
    }

    pub fn special_defense(self, attr: Attribute) -> usize {
        match attr {
            Attribute::Prowess => self.prowess,
            Attribute::Agility => self.agility,
            Attribute::Expertise => self.expertise,
            Attribute::Focus => self.focus,
            Attribute::Presence => self.presence,
        }
    }
}

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Defense {
    val: Option<usize>,
}
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct CritThreshold {
    val: usize,
}
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Efficacy {
    pub val: Ratio<usize>,
}

impl Mul<Efficacy> for usize {
    type Output = usize;

    fn mul(self, rhs: Efficacy) -> usize {
        let ratio: Ratio<usize> = self.into();
        (ratio * rhs.val).ceil().to_integer()
    }
}

/// Gets the attack bonus for the roll
pub fn get_attack_bonuses(
    mut attack_query: Query<(&mut AttackRoll, &Attacker, &AttackType), With<Active>>,
    attacker_query: Query<&AttackBonus>,
) {
    for (mut attack_roll, attacker, &attack_type) in attack_query.iter_mut() {
        let attack_bonuses = attacker_query.get(attacker.entity).unwrap();
        let attack_bonus = attack_bonuses.get(attack_type);

        if let Some(bonus) = attack_bonus {
            attack_roll.modifier = bonus
        }
    }
}

/// Rolls the attack
pub fn roll_attacks(mut query: Query<&mut AttackRoll, With<Active>>) {
    for mut attack_roll in query.iter_mut() {
        attack_roll.roll.roll();
    }
}

/// Attacks are cloned to each of the targets
pub fn dispatch_attacks(
    query: Query<(Entity, &Targets), (With<Attack>, With<Active>)>,
    mut _commands: Commands,
) {
    for (_attack_entity, targets) in query.iter() {
        for _target in targets.entities.iter() {
            // TODO: clone effects here
        }
    }
}

/// Gets the appropriate defense value for each defender
pub fn get_defenses(
    mut attack_query: Query<(&mut Defense, &Defender, &AttackType), With<Active>>,
    defender_query: Query<&Defenses>,
) {
    for (mut defense, defender, attack_type) in attack_query.iter_mut() {
        let defenses = defender_query.get(defender.entity).unwrap();

        defense.val = defenses.get(*attack_type);
    }
}

/// Marker component to note that an attack landed
#[derive(Component)]
pub struct Landed;

/// Marker component to note that an attack was a critical hit
#[derive(Component)]
pub struct Crit;

/// Each attack is checked to see if it landed
pub fn check_attacks(
    mut attacks: Query<(Entity, &mut Roll, &Defense, &CritThreshold), (With<Attack>, With<Active>)>,
    mut commands: Commands,
) {
    for (attack_entity, mut attack_roll, defense, crit_threshold) in attacks.iter_mut() {
        // Attacks where no Defense value is present always hit but never crit
        if defense.val.is_none() {
            commands.entity(attack_entity).insert(Landed);
            continue;
        }

        attack_roll.roll();

        if attack_roll.result.unwrap() >= defense.val.unwrap() {
            commands.entity(attack_entity).insert(Landed);

            if attack_roll.natural_roll.unwrap() >= crit_threshold.val {
                commands.entity(attack_entity).insert(Crit);
            }
        }
    }
}

/// The efficacy of attacks which were a critical hit are doubled
pub fn apply_crits(mut query: Query<&mut Efficacy, (With<Attack>, With<Active>, With<Crit>)>) {
    for mut efficacy in query.iter_mut() {
        efficacy.val *= 2;
    }
}

/// Efficacy doubles the effects of damage, absorption, afflictions, ailments and most forced movement
pub fn apply_efficacy(
    mut query: Query<
        (
            &Efficacy,
            Option<&mut DamageRoll>,
            Option<&mut Absorption>,
            Option<&mut Afflictions>,
            Option<&mut Ailments>,
            Option<&mut ForcedMovement>,
        ),
        (With<Attack>, With<Active>),
    >,
) {
    for (efficacy, damage_roll, absorption, afflictions, ailments, forced_movement) in
        query.iter_mut()
    {
        let efficacy = *efficacy;
        if let Some(mut dr) = damage_roll {
            *dr = dr.clone() * efficacy;
        }

        if let Some(mut ab) = absorption {
            *ab = ab.clone() * efficacy;
        }

        if let Some(mut af) = afflictions {
            *af = af.clone() * efficacy;
        }

        if let Some(mut ai) = ailments {
            *ai = ai.clone() * efficacy;
        }

        if let Some(mut fm) = forced_movement {
            *fm = fm.clone() * efficacy;
        }
    }
}
