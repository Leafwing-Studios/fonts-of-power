use crate::combat::attack::Efficacy;
use crate::core::skills::Skill;
use bevy::prelude::Component;
use derive_more::{Deref, DerefMut};
use num_rational::Ratio;
use std::collections::HashSet;
use std::ops::Mul;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Attribute {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Ideal {
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
pub struct ProficiencyBonus {
    val: i32,
}

#[derive(Clone, Debug, Eq, PartialEq, Deref, DerefMut)]
pub struct SkillProficiencies {
    val: HashSet<Skill>,
}

#[derive(Component, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Life {
    pub current: u16,
    pub max: u16,
}

#[derive(Component, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Absorption {
    pub val: u16,
}

impl Mul<Efficacy> for Absorption {
    type Output = Self;

    fn mul(self, rhs: Efficacy) -> Self {
        let lhs = Ratio::from_integer(self.val);

        let mut new = self.clone();
        new.val = (lhs * rhs.val).to_integer() as u16;
        new
    }
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
