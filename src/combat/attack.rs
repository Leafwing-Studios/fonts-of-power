use crate::combat::{
    actions::Targets,
    conditions::{Afflictions, Ailments},
    forced_movement::ForcedMovement,
    Active, Flow, Schedules,
};
use crate::core::dice::Roll;
use crate::core::stats::{Absorption, Attribute, Life};
use bevy::ecs::event::Events;
use bevy::ecs::schedule::SystemStage;
use bevy::prelude::*;
use bevy::utils::HashMap;
use num_rational::Ratio;
use std::cmp::max;
use std::ops::Mul;

pub struct AttackPlugin;
impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        use crate::combat::conditions::{resolve_afflictions, resolve_ailments};

        // Flows
        let mut roll_attack = SystemStage::parallel();
        let mut resolve_attack = SystemStage::parallel();

        roll_attack
            .add_system(get_attack_bonuses)
            .add_system(roll_attacks.after(get_attack_bonuses))
            .add_system(roll_damage.after(roll_attacks))
            .add_system(dispatch_attacks)
            .add_system(get_defenses.after(dispatch_attacks));

        resolve_attack
            .add_system(check_attacks)
            .add_system(roll_damage.after(check_attacks))
            .add_system(apply_resistances.after(roll_damage))
            .add_system(apply_crits.after(roll_damage).after(apply_resistances))
            .add_system(apply_efficacy.after(apply_crits).after(apply_resistances))
            .add_system(resolve_damage.after(apply_efficacy))
            .add_system(resolve_afflictions.after(apply_efficacy))
            .add_system(resolve_ailments.after(apply_efficacy));

        let mut schedules = app.world.resource_mut::<Schedules>();
        schedules.add_stage_as_flow(Flow::RollAttack, roll_attack);
        schedules.add_stage_as_flow(Flow::ResolveAttack, resolve_attack);

        // Events
        app.add_event::<LifeLost>();
    }
}

/// Fundamental component for [`Attack`] entities, which store the information about an attack's effects
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
#[allow(dead_code)]
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

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct DamageRoll {
    roll: Roll,
}

impl DamageRoll {
    pub fn apply_resistances(&mut self, damage_type: &DamageType, resistances: &Resistances) {
        let current_damage: Ratio<usize> = Ratio::from_integer(self.roll.result.unwrap() as usize);
        let new_damage = match damage_type {
            DamageType::Pure(e) => current_damage * resistances.get(e).damage_multiplier(),
            DamageType::Hybrid(e1, e2) => {
                current_damage
                    * max(
                        resistances.get(e1).damage_multiplier(),
                        resistances.get(e2).damage_multiplier(),
                    )
            }
            DamageType::Split(e1, e2) => {
                let half_damage = Ratio::new(1, 2) * current_damage;
                let partial_1 = half_damage * resistances.get(e1).damage_multiplier();
                let partial_2 = half_damage * resistances.get(e2).damage_multiplier();

                // Rounding up is equivalent to making the remainder count as the more effective damage type
                partial_1 + partial_2
            }
        };

        self.roll.result = Some(new_damage.round().to_integer());
    }
}

impl Mul<Efficacy> for DamageRoll {
    type Output = Self;

    fn mul(self, rhs: Efficacy) -> Self {
        let product = self.roll.result.unwrap() * rhs;

        DamageRoll {
            roll: Roll {
                result: Some(product),
                ..self.roll
            },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum Element {
    Physical,
    Air,
    Earth,
    Fire,
    Water,
    Radiant,
    Umbral,
    Primal,
    Decay,
    Electric,
    Corrosive,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum ResistanceLevel {
    Vulnerable,
    Normal,
    Resistant,
    Immune,
}

impl Default for ResistanceLevel {
    fn default() -> Self {
        ResistanceLevel::Normal
    }
}

impl Default for &ResistanceLevel {
    fn default() -> Self {
        &ResistanceLevel::Normal
    }
}

impl ResistanceLevel {
    pub fn damage_multiplier(self) -> Ratio<usize> {
        match self {
            Self::Vulnerable => Ratio::new(2, 1),
            Self::Normal => Ratio::new(1, 1),
            Self::Resistant => Ratio::new(1, 2),
            Self::Immune => Ratio::new(0, 1),
        }
    }
}
#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Resistances(HashMap<Element, ResistanceLevel>);

impl Resistances {
    pub fn get(&self, element: &Element) -> ResistanceLevel {
        match self.0.get(element) {
            Some(rl) => *rl,
            None => ResistanceLevel::Normal,
        }
    }
}
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum DamageType {
    Pure(Element),
    Hybrid(Element, Element),
    Split(Element, Element),
}

impl DamageType {}

pub fn roll_damage(mut query: Query<&mut DamageRoll, With<Active>>) {
    for mut damage_roll in query.iter_mut() {
        damage_roll.roll.roll();
    }
}

pub fn apply_resistances(
    mut damage_query: Query<(&Defender, &DamageType, &mut DamageRoll), With<Active>>,
    resistance_query: Query<&Resistances>,
) {
    for (defender, damage_type, mut damage) in damage_query.iter_mut() {
        let resistances = resistance_query.get(defender.entity).unwrap();

        damage.apply_resistances(damage_type, resistances);
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
/// An event which records the loss of life
pub struct LifeLost {
    defender: Defender,
}

pub fn resolve_damage(
    damage_query: Query<(&DamageRoll, &Defender), With<Active>>,
    mut life_query: Query<(&mut Life, &mut Absorption)>,
    mut life_lost: ResMut<Events<LifeLost>>,
) {
    for (damage, defender) in damage_query.iter() {
        let (mut life, mut absorption) = life_query.get_mut(defender.entity).unwrap();

        let damage_dealt = damage.roll.result.unwrap();

        if absorption.val > damage_dealt {
            absorption.val -= damage_dealt;
        } else {
            life.current -= damage_dealt - absorption.val;
            absorption.val = 0;

            life_lost.send(LifeLost {
                defender: defender.clone(),
            });
        }
    }
}
