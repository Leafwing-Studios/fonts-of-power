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
#[derive(Clone, Deref, DerefMut)]
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

#[derive(Clone, Deref, DerefMut)]
pub struct Ailments(HashMap<Ailments, u16>);

#[non_exhaustive]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Status {
    DeathsDoor,
    Flying,
    Grappled,
    Grappling,
    Hidden,
    Invisible,
    Prone,
    Unconscious,
}
