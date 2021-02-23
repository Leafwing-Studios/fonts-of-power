use derive_more::{Deref, DerefMut};
use std::{collections::HashSet, marker::PhantomData};

use crate::core::skills::Skill;

#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Attribute {
    Any,
    Prowess,
    Agility,
    Expertise,
    Focus,
    Presence,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct AttributeVal(i8);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
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

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct IdealVal(i8);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Ideals {
    equality: IdealVal,
    harmony: IdealVal,
    liberty: IdealVal,
    progress: IdealVal,
    sanctity: IdealVal,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct ProficiencyBonus(i8);

#[derive(Clone, Debug, Eq, PartialEq, Deref, DerefMut)]
pub struct Proficiencies(HashSet<Skill>);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct Life(u16);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct Essence(u16);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct Exhaustion(u16);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct Level(u8);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct Tier(u8);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct BasicAttackBonus(i8);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct SpecialAttackBonus(i8);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct BasicDefense(i8);

// TODO: determine a better approach for constructing these and enforcing that T is an Attribute variant
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct SpecialDefense<T> {
    val: i8,
    phantom: PhantomData<T>,
}
