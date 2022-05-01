use crate::combat::attack::{Defender, Efficacy};
use crate::combat::Active;
use crate::core::dice::Roll;
use crate::core::stats::{Absorption, Life};
use bevy::ecs::Events;
use bevy::prelude::{Component, Query, ResMut, With};
use bevy::utils::HashMap;
use num_rational::Ratio;
use std::cmp::max;
use std::ops::Mul;

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
#[non_exhaustive]
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

#[derive(Clone)]
pub struct LifeLost {
    defender: Defender,
}

pub fn apply_damage(
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

// TODO: check life lost events for concentrating entities
