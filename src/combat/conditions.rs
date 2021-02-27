use derive_more::{Deref, DerefMut};
use std::collections::HashMap;

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
#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Afflictions(HashMap<Affliction, u16>);

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

#[derive(Clone, Debug, Deref, DerefMut)]
pub struct Ailments(HashMap<Ailments, u16>);

pub fn apply_afflictions() {}

pub fn apply_ailments() {}

pub trait Status {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct DeathsDoor;
impl Status for DeathsDoor {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Flying;
impl Status for Flying {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Grappled;
impl Status for Grappled {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Grappling;
impl Status for Grappling {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Hidden;
impl Status for Hidden {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Invisible;
impl Status for Invisible {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Prone;
impl Status for Prone {}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Unconscious;
impl Status for Unconscious {}
