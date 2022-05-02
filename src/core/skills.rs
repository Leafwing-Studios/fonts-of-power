use crate::core::dice::Roll;
use crate::core::stats::Attribute;
use bevy::prelude::Component;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[allow(dead_code)]
pub enum Skill {
    Anima,
    Athletics,
    Charm,
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
#[allow(dead_code)]
pub enum SkillCheckOutcome {
    Failure,
    MixedSuccess,
    Success,
    SmashingSuccess,
}

#[derive(Clone, Copy, Debug, PartialEq, Component)]
pub struct SkillCheck {
    attribute: Attribute,
    skill: Skill,
    actor_roll: Option<Roll>,
    flat_difficulty: Option<usize>,
    opposed_roll: Option<Roll>,
    combat: bool,
    proficient: bool,
}

impl SkillCheck {
    #[allow(dead_code)]
    pub fn roll(&self) -> SkillCheckOutcome {
        self.actor_roll.unwrap().roll();

        let result = self.actor_roll.unwrap().result.unwrap();
        let difficulty = if self.opposed_roll.is_some() {
            self.opposed_roll.unwrap().roll();
            self.opposed_roll.unwrap().result.unwrap()
        } else {
            self.flat_difficulty.unwrap()
        };

        if self.combat {
            match result {
                r if r >= difficulty => SkillCheckOutcome::Success,
                _ => SkillCheckOutcome::Failure,
            }
        } else {
            match result {
                r if r >= difficulty + 5 => SkillCheckOutcome::SmashingSuccess,
                r if r >= difficulty => SkillCheckOutcome::Success,
                r if (r >= difficulty - 5) | self.proficient => SkillCheckOutcome::MixedSuccess,
                _ => SkillCheckOutcome::Failure,
            }
        }
    }
}
