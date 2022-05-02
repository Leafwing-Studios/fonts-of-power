use crate::combat::attack::Efficacy;
use bevy::prelude::Component;
use bevy::utils::HashMap;
use num_rational::Ratio;
use std::ops::Mul;

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Affliction {
    Aflame,
    Anguish,
    Bleeding,
    Brittle,
    Rage,
    Shock,
    Temperance,
    Unstable,
}

#[derive(Component, Clone, Debug)]
pub struct Afflictions {
    map: HashMap<Affliction, usize>,
}

impl Mul<Efficacy> for Afflictions {
    type Output = Self;

    fn mul(mut self, rhs: Efficacy) -> Self {
        for (_k, v) in self.map.iter_mut() {
            let lhs = Ratio::from_integer(*v);
            *v = (lhs * rhs.val).to_integer() as usize;
        }

        self
    }
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Ailment {
    Aloft,
    Banished,
    Blinded,
    Blunted,
    Chilled,
    Crazed,
    Cursed,
    Defenseless,
    Frightened,
    Hapless,
    Pacified,
    Rooted,
    Taunted,
    Withered,
}

#[derive(Component, Clone, Debug)]
pub struct Ailments {
    map: HashMap<Ailment, usize>,
}

impl Mul<Efficacy> for Ailments {
    type Output = Self;

    fn mul(mut self, rhs: Efficacy) -> Self {
        for (_k, v) in self.map.iter_mut() {
            let lhs = Ratio::from_integer(*v);
            *v = (lhs * rhs.val).to_integer() as usize;
        }

        self
    }
}

pub fn resolve_afflictions() {}

pub fn resolve_ailments() {}

pub trait Status: Component {}

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct DeathsDoor;
impl Status for DeathsDoor {}

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Flying;
impl Status for Flying {}

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Grappled;
impl Status for Grappled {}

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Grappling;
impl Status for Grappling {}

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Hidden;
impl Status for Hidden {}

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Invisible;
impl Status for Invisible {}

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Prone;
impl Status for Prone {}

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Unconscious;
impl Status for Unconscious {}
