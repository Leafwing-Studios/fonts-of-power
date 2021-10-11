use crate::combat::actions::Targets;
use crate::combat::conditions::{Afflictions, Ailments};
use crate::combat::damage::DamageRoll;
use crate::combat::forced_movement::ForcedMovement;
use crate::combat::Active;
use crate::core::dice::Roll;
use crate::core::stats::{Absorption, Attribute};
use bevy::prelude::Component;
use bevy::prelude::{Commands, Entity, Query, With};
use derive_more::{Deref, DerefMut};
use num_rational::Ratio;

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

#[derive(Component, Clone, Debug, Deref, DerefMut, PartialEq, Eq)]
pub struct Attacker(Entity);

#[derive(Component, Clone, Debug, Deref, DerefMut, PartialEq, Eq)]
pub struct Defender(Entity);

#[derive(Component, Clone, Debug, PartialEq, Eq, Deref, DerefMut)]
pub struct AttackRoll(Roll);
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq)]
pub enum AttackType {
    Basic,
    Special(Attribute),
    NoRoll,
}

#[derive(Component, Copy, Clone, Debug)]
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

#[derive(Component, Copy, Clone, Debug)]
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

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Defense {
    val: Option<i32>,
}
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct CritThreshold {
    val: u8,
}
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct Efficacy {
    pub val: Ratio<u16>,
}

/// Gets the attack bonus for the roll
pub fn get_attack_bonuses(
    mut attack_query: Query<(&mut AttackRoll, &Attacker, &AttackType), With<Active>>,
    attacker_query: Query<&AttackBonus>,
) {
    for (mut attack_roll, attacker, attack_type) in attack_query.iter_mut() {
        let attack_bonuses = attacker_query.get(**attacker).unwrap();
        let attack_bonus = attack_bonuses.get(*attack_type);

        if let Some(bonus) = attack_bonus {
            attack_roll.modifier = bonus
        }
    }
}

/// Rolls the attack
pub fn roll_attacks(mut query: Query<&mut AttackRoll, With<Active>>) {
    for attack_roll in query.iter_mut() {
        attack_roll.roll();
    }
}

/// Attacks are cloned to each of the targets
pub fn dispatch_attacks(
    query: Query<(Entity, &Targets), (With<Attack>, With<Active>)>,
    mut commands: Commands,
) {
    for (attack_entity, targets) in query.iter() {
        for target in targets.iter() {
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
        let defenses = defender_query.get(**defender).unwrap();

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
    attacks: Query<(Entity, &Roll, &Defense, &CritThreshold), (With<Attack>, With<Active>)>,
    mut commands: Commands,
) {
    for (attack_entity, attack_roll, defense, crit_threshold) in attacks.iter() {
        // Attacks where no Defense value is present always hit but never crit
        if defense.val.is_none() {
            commands.entity(attack_entity).insert(Landed);
            continue;
        }

        attack_roll.roll();

        if attack_roll.result().unwrap() >= defense.val.unwrap() {
            commands.entity(attack_entity).insert(Landed);

            if attack_roll.natural_roll().unwrap() as u8 >= crit_threshold.val {
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
