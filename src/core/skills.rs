use crate::core::dice::Roll;
use crate::core::stats::Attribute;

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

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum SkillCheckOutcome {
    Failure,
    MixedSuccess,
    Success,
    SmashingSuccess,
}
#[allow(dead_code)]
pub struct SkillCheck {
    attribute: Attribute,
    skill: Skill,
    roll: Option<Roll>,
    flat_difficulty: Option<i32>,
    opposed_roll: Option<Roll>,
    combat: bool,
    proficient: bool,
}

impl SkillCheck {
    #[allow(dead_code)]
    pub fn roll(&self) -> SkillCheckOutcome {
        let result = self.roll.unwrap().roll();

        let realized_difficulty: i32 = if let Some(flat_difficulty) = self.flat_difficulty {
            flat_difficulty
        } else if let Some(opposed_difficulty) = self.opposed_roll {
            opposed_difficulty.roll()
        } else {
            panic!("No difficulty found for skill check.")
        };

        if self.combat {
            if result >= realized_difficulty {
                return SkillCheckOutcome::Success;
            } else {
                return SkillCheckOutcome::Failure;
            }
        } else {
            if result >= realized_difficulty + 5 {
                return SkillCheckOutcome::SmashingSuccess;
            } else if result >= realized_difficulty {
                return SkillCheckOutcome::Success;
            } else if (result >= realized_difficulty - 5) | self.proficient {
                return SkillCheckOutcome::MixedSuccess;
            } else {
                return SkillCheckOutcome::Failure;
            }
        }
    }
}
