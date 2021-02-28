use crate::combat::attack::Defender;
use crate::combat::Active;
use crate::core::dice::Roll;
use crate::core::stats::Life;
use bevy::app::Events;
use bevy::ecs::{Query, ResMut, With};
use derive_more::{Deref, DerefMut};
use num_rational::Ratio;
use std::cmp::max;
use std::collections::HashMap;
use std::ops::Mul;

#[derive(Clone, PartialEq, Eq)]
pub struct DamageRoll {
    roll: Roll,
    result: Option<u16>,
}

impl Mul<Ratio<u16>> for DamageRoll {
    type Output = Self;

    fn mul(self, rhs: Ratio<u16>) -> Self {
        let ratio = Ratio::from_integer(self.result.unwrap()) * rhs;
        DamageRoll {
            roll: self.roll,
            result: Some(ratio.to_integer()),
        }
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
    Eldritch,
    Arcane,
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
#[derive(Clone, Debug, PartialEq, Eq, Deref, DerefMut)]
pub struct Resistances(HashMap<Element, ResistanceLevel>);
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DamageType {
    Pure(Element),
    Hybrid(Element, Element),
    Split(Element, Element),
}

impl DamageType {
    pub fn calculate_damage_with_resistances(
        self,
        damage: &DamageRoll,
        resistances: &Resistances,
    ) -> DamageRoll {
        match self {
            DamageType::Pure(e) => {
                damage.clone() * resistances.get(&e).unwrap().damage_multiplier()
            }
            DamageType::Hybrid(e1, e2) => {
                damage.clone()
                    * max(
                        resistances.get(&e1).unwrap().damage_multiplier(),
                        resistances.get(&e2).unwrap().damage_multiplier(),
                    )
            }
            DamageType::Split(e1, e2) => {
                let half_damage = Ratio::from_integer(damage.result.unwrap()) / 2;
                let partial_1 = half_damage * resistances.get(&e1).unwrap().damage_multiplier();
                let partial_2 = half_damage * resistances.get(&e2).unwrap().damage_multiplier();

                // Rounding up is equivalent to making the remainder count as the more effective damage type
                let total = (partial_1 + partial_2).round();
                DamageRoll {
                    roll: damage.roll,
                    result: Some(total.to_integer()),
                }
            }
        }
    }
}

pub fn roll_damage(mut query: Query<&mut DamageRoll, With<Active>>) {
    for mut i in query.iter_mut() {
        i.result = Some(i.roll.roll() as u16);
    }
}

pub fn apply_resistances(
    mut damage_query: Query<(&Defender, &DamageType, &mut DamageRoll), With<Active>>,
    resistance_query: Query<&Resistances>,
) {
    for (defender, damage_type, mut damage) in damage_query.iter_mut() {
        let resistances = resistance_query.get(**defender).unwrap();

        *damage = damage_type.calculate_damage_with_resistances(&*damage, resistances);
    }
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct LifeLost {
    defender: Defender,
}

pub fn apply_damage(
    damage_query: Query<(&DamageRoll, &Defender), With<Active>>,
    mut life_query: Query<&mut Life>,
    mut life_lost: ResMut<Events<LifeLost>>,
) {
    for (damage, defender) in damage_query.iter() {
        let mut life = life_query.get_mut(**defender).unwrap();

        let damage_dealt = damage.result.unwrap();

        if life.absorption > damage_dealt {
            life.absorption = life.absorption - damage_dealt;
        } else {
            life.absorption = 0;
            life.current -= damage_dealt - damage_dealt;

            life_lost.send(LifeLost {
                defender: defender.clone(),
            });
        }
    }
}

// TODO: check life lost events for concentrating entities
