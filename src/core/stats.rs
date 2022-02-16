use crate::combat::attack::Efficacy;
use crate::core::skills::Skill;
use bevy::prelude::Component;
use derive_more::{Deref, DerefMut};
use num_rational::Ratio;
use std::collections::HashSet;
use std::ops::Mul;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Attribute {
    Prowess,
    Agility,
    Expertise,
    Focus,
    Presence,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct AttributeVal(isize);

#[derive(Component, Clone, Debug)]
pub struct Attributes {
    prowess: AttributeVal,
    agility: AttributeVal,
    expertise: AttributeVal,
    focus: AttributeVal,
    presence: AttributeVal,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ideal {
    Equality,
    Harmony,
    Liberty,
    Progress,
    Sanctity,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct IdealVal(isize);

#[derive(Component, Clone, Debug)]
pub struct Ideals {
    equality: IdealVal,
    harmony: IdealVal,
    liberty: IdealVal,
    progress: IdealVal,
    sanctity: IdealVal,
}

#[derive(Component, Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct ProficiencyBonus {
    val: isize,
}

#[derive(Component, Clone, Debug, Eq, PartialEq, Deref, DerefMut)]
pub struct SkillProficiencies {
    val: HashSet<Skill>,
}

#[derive(Component, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Life {
    pub current: usize,
    pub max: usize,
}

#[derive(Component, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Absorption {
    pub val: usize,
}

impl Mul<Efficacy> for Absorption {
    type Output = Self;

    fn mul(self, rhs: Efficacy) -> Self {
        let lhs = Ratio::from_integer(self.val);

        Absorption {
            val: (lhs * rhs.val).to_integer() as usize,
        }
    }
}

#[derive(Component, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Essence {
    pub current: usize,
    pub max: usize,
}
#[derive(Component, Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct Exhaustion(pub usize);

#[derive(Component, Clone, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct Level(pub usize);
