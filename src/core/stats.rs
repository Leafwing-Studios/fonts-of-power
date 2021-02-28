use derive_more::{Deref, DerefMut};
use std::collections::HashSet;

use crate::core::skills::Skill;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Attribute {
    Any,
    Prowess,
    Agility,
    Expertise,
    Focus,
    Presence,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct AttributeVal(i8);

#[derive(Clone, Debug)]
pub struct Attributes {
    prowess: AttributeVal,
    agility: AttributeVal,
    expertise: AttributeVal,
    focus: AttributeVal,
    presence: AttributeVal,
}
#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Ideal {
    Any,
    Equality,
    Harmony,
    Liberty,
    Progress,
    Sanctity,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct IdealVal(i8);

#[derive(Clone, Debug)]
pub struct Ideals {
    equality: IdealVal,
    harmony: IdealVal,
    liberty: IdealVal,
    progress: IdealVal,
    sanctity: IdealVal,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct ProficiencyBonus(i8);

#[derive(Clone, Debug, Eq, PartialEq, Deref, DerefMut)]
pub struct SkillProficiencies(HashSet<Skill>);

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Life {
    pub current: u16,
    pub max: u16,
    pub absorption: u16,
}
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Essence {
    pub current: u16,
    pub max: u16,
}
#[derive(Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct Exhaustion(pub u16);

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct Level(pub u8);

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct Tier(pub u8);

#[derive(Clone, Debug)]
pub struct AttackBonus {
    basic: i8,
    special: i8,
}

#[derive(Clone, Debug)]
pub struct Defenses {
    basic: i8,
    prowess: i8,
    agility: i8,
    expertise: i8,
    focus: i8,
    presence: i8,
}
