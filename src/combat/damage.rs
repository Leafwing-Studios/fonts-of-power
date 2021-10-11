use crate::combat::attack::{Defender, Efficacy};
use crate::combat::Active;
use crate::core::dice::Roll;
use crate::core::stats::{Absorption, Life};
use bevy::app::Events;
use bevy::prelude::{Component, Query, ResMut, With};
use derive_more::{Deref, DerefMut};
use num_rational::Ratio;
use std::cmp::max;
use std::collections::HashMap;
use std::ops::Mul;

#[derive(Component, Clone, Debug, PartialEq, Eq, Deref, DerefMut)]
pub struct DamageRoll(Roll);

impl DamageRoll {
    pub fn apply_resistances(&self, damage_type: &DamageType, resistances: &Resistances) {
        let current_damage: Ratio<u16> = Ratio::from_integer(self.result().unwrap() as u16);
        let new_damage = match damage_type {
            DamageType::Pure(e) => current_damage * resistances.get(&e).damage_multiplier(),
            DamageType::Hybrid(e1, e2) => {
                current_damage
                    * max(
                        resistances.get(&e1).damage_multiplier(),
                        resistances.get(&e2).damage_multiplier(),
                    )
            }
            DamageType::Split(e1, e2) => {
                let half_damage = Ratio::new(1, 2) * current_damage;
                let partial_1 = half_damage * resistances.get(&e1).damage_multiplier();
                let partial_2 = half_damage * resistances.get(&e2).damage_multiplier();

                // Rounding up is equivalent to making the remainder count as the more effective damage type
                partial_1 + partial_2
            }
        };

        self.set_result(new_damage.round().to_integer() as i32);
    }
}

impl Mul<Efficacy> for DamageRoll {
    type Output = Self;

    fn mul(self, rhs: Efficacy) -> Self {
        let lhs = Ratio::from_integer(self.result().unwrap() as u16);

        let new = self.clone();
        new.set_result((lhs * rhs.val).to_integer() as i32);
        new
    }
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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
    pub fn damage_multiplier(self) -> Ratio<u16> {
        match self {
            Self::Vulnerable => Ratio::new(2, 1),
            Self::Normal => Ratio::new(1, 1),
            Self::Resistant => Ratio::new(1, 2),
            Self::Immune => Ratio::new(0, 1),
        }
    }
}
#[allow(dead_code)]
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
#[allow(dead_code)]
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DamageType {
    Pure(Element),
    Hybrid(Element, Element),
    Split(Element, Element),
}

impl DamageType {}

pub fn roll_damage(mut query: Query<&mut DamageRoll, With<Active>>) {
    for damage_roll in query.iter_mut() {
        damage_roll.roll();
    }
}

pub fn apply_resistances(
    mut damage_query: Query<(&Defender, &DamageType, &mut DamageRoll), With<Active>>,
    resistance_query: Query<&Resistances>,
) {
    for (defender, damage_type, damage) in damage_query.iter_mut() {
        let resistances = resistance_query.get(**defender).unwrap();

        damage.apply_resistances(damage_type, resistances);
    }
}

#[allow(dead_code)]
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
        let (mut life, mut absorption) = life_query.get_mut(**defender).unwrap();

        let damage_dealt = damage.result().unwrap() as u16;

        if absorption.val > damage_dealt {
            absorption.val = absorption.val - damage_dealt;
        } else {
            absorption.val = 0;
            life.current -= damage_dealt - damage_dealt;

            life_lost.send(LifeLost {
                defender: defender.clone(),
            });
        }
    }
}

// TODO: check life lost events for concentrating entities
