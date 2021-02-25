use crate::core::dice::{roll, Advantage, DieSize, NumDice};
use crate::core::stats::AttributeVal;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Skill {
    Anima,
    Arcana,
    Athletics,
    Charm,
    Craftsmanship,
    Endurance,
    Fontcraft,
    Guidance,
    Humanities,
    Insight,
    Medicine,
    Perception,
    Stealth,
    Tinkering,
    Trickery,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum SkillCheckOutcome {
    Failure,
    MixedSuccess,
    Success,
    SmashingSuccess,
}

pub trait SkillCheck {
    fn roll(&self) -> SkillCheckOutcome;
}
#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct SimpleSkillCheck {
    difficulty: i8,
    attribute: AttributeVal,
    skill: Skill,
    advantage: Advantage,
    proficient: bool,
    proficiency_bonus: i8,
}

impl SkillCheck for SimpleSkillCheck {
    fn roll(&self) -> SkillCheckOutcome {
        let mut result = roll(NumDice(1), DieSize::d20, self.advantage) as i8 + *self.attribute;

        if self.proficient {
            result += self.proficiency_bonus;
        }

        if result >= self.difficulty + 5 {
            return SkillCheckOutcome::SmashingSuccess;
        } else if result >= self.difficulty {
            return SkillCheckOutcome::Success;
        } else if (result >= self.difficulty - 5) | self.proficient {
            return SkillCheckOutcome::MixedSuccess;
        } else {
            return SkillCheckOutcome::Failure;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CombatSkillCheck {
    difficulty: i8,
    attribute: AttributeVal,
    skill: Skill,
    advantage: Advantage,
    proficient: bool,
    proficiency_bonus: i8,
}

impl SkillCheck for CombatSkillCheck {
    fn roll(&self) -> SkillCheckOutcome {
        let mut result = roll(NumDice(1), DieSize::d20, self.advantage) as i8 + *self.attribute;

        if self.proficient {
            result += self.proficiency_bonus;
        }

        if result >= self.difficulty {
            return SkillCheckOutcome::Success;
        } else {
            return SkillCheckOutcome::Failure;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct OpposedSkillCheck {
    attribute_a: AttributeVal,
    skill_a: Skill,
    advantage_a: Advantage,
    proficient_a: bool,
    proficiency_bonus_a: i8,
    attribute_b: AttributeVal,
    skill_b: Skill,
    advantage_b: Advantage,
    proficient_b: bool,
    proficiency_bonus_b: i8,
}

impl SkillCheck for OpposedSkillCheck {
    fn roll(&self) -> SkillCheckOutcome {
        let mut result = roll(NumDice(1), DieSize::d20, self.advantage_a) as i8 + *self.attribute_a;

        if self.proficient_a {
            result += self.proficiency_bonus_a;
        }

        let mut difficulty =
            roll(NumDice(1), DieSize::d20, self.advantage_b) as i8 + *self.attribute_b;

        if self.proficient_b {
            difficulty += self.proficiency_bonus_b;
        }

        if result >= difficulty {
            return SkillCheckOutcome::Success;
        } else {
            return SkillCheckOutcome::Failure;
        }
    }
}
