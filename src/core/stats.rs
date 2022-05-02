use crate::combat::attack::Efficacy;
use crate::core::skills::Skill;
use bevy::prelude::Component;
use bounded_integer::{BoundedI8, BoundedU8};
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

#[derive(Component, Clone, Debug)]
#[allow(dead_code)]
pub struct Attributes {
    values: [BoundedI8<-5, 5>; 5],
}

impl Attributes {
    #[allow(dead_code)]
    pub fn get(&self, attribute: Attribute) -> BoundedI8<-5, 5> {
        match attribute {
            Attribute::Prowess => self.prowess(),
            Attribute::Agility => self.agility(),
            Attribute::Expertise => self.expertise(),
            Attribute::Focus => self.focus(),
            Attribute::Presence => self.presence(),
        }
    }

    #[allow(dead_code)]
    pub fn prowess(&self) -> BoundedI8<-5, 5> {
        self.values[0]
    }

    #[allow(dead_code)]
    pub fn agility(&self) -> BoundedI8<-5, 5> {
        self.values[1]
    }

    #[allow(dead_code)]
    pub fn expertise(&self) -> BoundedI8<-5, 5> {
        self.values[2]
    }

    #[allow(dead_code)]
    pub fn focus(&self) -> BoundedI8<-5, 5> {
        self.values[3]
    }

    #[allow(dead_code)]
    pub fn presence(&self) -> BoundedI8<-5, 5> {
        self.values[4]
    }
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

#[derive(Component, Clone, Debug)]
pub struct Ideals {
    #[allow(dead_code)]
    values: [BoundedU8<0, 5>; 5],
}

impl Ideals {
    #[allow(dead_code)]
    pub fn get(&self, ideal: Ideal) -> BoundedU8<0, 5> {
        match ideal {
            Ideal::Equality => self.equality(),
            Ideal::Harmony => self.harmony(),
            Ideal::Liberty => self.liberty(),
            Ideal::Progress => self.progress(),
            Ideal::Sanctity => self.sanctity(),
        }
    }

    #[allow(dead_code)]
    pub fn equality(&self) -> BoundedU8<0, 5> {
        self.values[0]
    }

    #[allow(dead_code)]
    pub fn harmony(&self) -> BoundedU8<0, 5> {
        self.values[1]
    }

    #[allow(dead_code)]
    pub fn liberty(&self) -> BoundedU8<0, 5> {
        self.values[2]
    }

    #[allow(dead_code)]
    pub fn progress(&self) -> BoundedU8<0, 5> {
        self.values[3]
    }

    #[allow(dead_code)]
    pub fn sanctity(&self) -> BoundedU8<0, 5> {
        self.values[4]
    }
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
    level: BoundedU8<0, 10>,
}
