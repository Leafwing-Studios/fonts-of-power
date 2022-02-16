use bevy::prelude::Component;
use rand::Rng;
use std::cmp::{max, min};
use std::convert::TryFrom;

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

impl TryFrom<isize> for DieSize {
    type Error = &'static str;

    fn try_from(d: isize) -> Result<Self, Self::Error> {
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

#[derive(Component, Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Roll {
    pub n: isize,
    pub d: DieSize,
    pub advantage: Advantage,
    pub modifier: isize,
    natural_roll: Option<isize>,
    result: Option<isize>,
}

impl Roll {
    fn roll_once(n: isize, d: DieSize) -> isize {
        let mut rng = rand::thread_rng();

        (0..n).map(|_| rng.gen_range(1..=d as isize)).sum()
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

    pub fn natural_roll(self) -> Option<isize> {
        self.natural_roll
    }

    pub fn result(self) -> Option<isize> {
        self.result
    }

    /// This should only be called when directly modifying a rolled result. Prefer Roll::roll()
    pub fn set_result(mut self, new_result: isize) {
        self.result = Some(new_result);
    }
}
