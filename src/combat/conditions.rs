use crate::combat::attack::Efficacy;
use bevy::prelude::Component;
use num_rational::Ratio;
use std::collections::HashMap;
use std::ops::Mul;

#[non_exhaustive]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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

#[allow(dead_code)]
#[derive(Component, Clone, Debug)]
pub struct Afflictions {
    map: HashMap<Affliction, u16>,
}

impl Mul<Efficacy> for Afflictions {
    type Output = Self;

    fn mul(self, rhs: Efficacy) -> Self {
        let mut new = self.clone();
        for (_k, v) in new.map.iter_mut() {
            let lhs = Ratio::from_integer(*v);
            *v = (lhs * rhs.val).to_integer() as u16;
        }

        new
    }
}

#[non_exhaustive]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
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

#[allow(dead_code)]
#[derive(Component, Clone, Debug)]
pub struct Ailments {
    map: HashMap<Ailment, u16>,
}

impl Mul<Efficacy> for Ailments {
    type Output = Self;

    fn mul(self, rhs: Efficacy) -> Self {
        let mut new = self.clone();
        for (_k, v) in new.map.iter_mut() {
            let lhs = Ratio::from_integer(*v);
            *v = (lhs * rhs.val).to_integer() as u16;
        }

        new
    }
}

pub fn apply_afflictions() {}

pub fn apply_ailments() {}

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
