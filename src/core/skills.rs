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
    actor_roll: Option<Roll>,
    flat_difficulty: Option<i32>,
    opposed_roll: Option<Roll>,
    combat: bool,
    proficient: bool,
}

impl SkillCheck {
    #[allow(dead_code)]
    pub fn roll(&self) -> SkillCheckOutcome {
        self.actor_roll.unwrap().roll();

        let result = self.actor_roll.unwrap().result().unwrap();
        let difficulty = if self.opposed_roll.is_some() {
            self.opposed_roll.unwrap().roll();
            self.opposed_roll.unwrap().result().unwrap()
        } else {
            self.flat_difficulty.unwrap()
        };

        if self.combat {
            if result >= difficulty {
                return SkillCheckOutcome::Success;
            } else {
                return SkillCheckOutcome::Failure;
            }
        } else {
            if result >= difficulty + 5 {
                return SkillCheckOutcome::SmashingSuccess;
            } else if result >= difficulty {
                return SkillCheckOutcome::Success;
            } else if (result >= difficulty - 5) | self.proficient {
                return SkillCheckOutcome::MixedSuccess;
            } else {
                return SkillCheckOutcome::Failure;
            }
        }
    }
}
