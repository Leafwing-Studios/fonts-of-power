use crate::combat::attack::Efficacy;
use crate::core::skills::Skill;
use bevy::prelude::Component;
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

#[derive(Component, Clone, Debug)]
pub struct Attributes {
    prowess: isize,
    agility: isize,
    expertise: isize,
    focus: isize,
    presence: isize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ideal {
    Equality,
    Harmony,
    Liberty,
    Progress,
    Sanctity,
}

#[derive(Component, Clone, Debug)]
pub struct Ideals {
    equality: isize,
    harmony: isize,
    liberty: isize,
    progress: isize,
    sanctity: isize,
}

#[derive(Component, Clone, Debug, Hash, Eq, PartialEq)]
pub struct ProficiencyBonus {
    val: isize,
}

#[derive(Component, Clone, Debug, Eq, PartialEq)]
pub struct SkillProficiencies {
    set: HashSet<Skill>,
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
#[derive(Component, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Exhaustion {
    pub stacks: usize,
}

#[derive(Component, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Level {
    level: usize,
}
