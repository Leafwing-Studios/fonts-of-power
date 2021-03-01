use bevy::math::i32;
use rand::Rng;
use std::cmp::{max, min};
use std::convert::TryFrom;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Advantage {
    Disadvantage,
    Neutral,
    Advantage,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub enum DieSize {
    d4,
    d6,
    d8,
    d10,
    d12,
    d20,
    d100,
}

impl TryFrom<i8> for DieSize {
    type Error = &'static str;

    fn try_from(d: i8) -> Result<Self, Self::Error> {
        match d {
            4 => Ok(DieSize::d4),
            6 => Ok(DieSize::d6),
            8 => Ok(DieSize::d8),
            10 => Ok(DieSize::d10),
            12 => Ok(DieSize::d12),
            20 => Ok(DieSize::d20),
            100 => Ok(DieSize::d100),
            _ => Err("Invalid die size"),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Roll {
    pub n: i32,
    pub d: DieSize,
    pub advantage: Advantage,
    pub modifier: i32,
    natural_roll: Option<i32>,
    result: Option<i32>,
}

impl Roll {
    fn roll_once(n: i32, d: DieSize) -> i32 {
        let mut rng = rand::thread_rng();

        (0..n).map(|_| rng.gen_range(1..=d as i32)).sum()
    }

    pub fn roll(mut self) {
        self.natural_roll = Some(match self.advantage {
            Advantage::Disadvantage => min(
                Roll::roll_once(self.n, self.d),
                Roll::roll_once(self.n, self.d),
            ),
            Advantage::Neutral => Roll::roll_once(self.n, self.d),
            Advantage::Advantage => max(
                Roll::roll_once(self.n, self.d),
                Roll::roll_once(self.n, self.d),
            ),
        });

        self.result = Some(self.natural_roll.unwrap() + self.modifier);
    }

    pub fn natural_roll(self) -> Option<i32> {
        self.natural_roll
    }

    pub fn result(self) -> Option<i32> {
        self.result
    }

    /// This should only be called when directly modifying a rolled result. Prefer Roll::roll()
    pub fn set_result(mut self, new_result: i32) {
        self.result = Some(new_result);
    }
}
