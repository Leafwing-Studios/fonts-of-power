use bevy::prelude::Component;
use std::cmp::{max, min};
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
#[allow(dead_code)]
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
    /// How many dice should be rolled?
    pub n: isize,
    /// How many sides do the dice rolled have?
    pub d: DieSize,
    /// Should the higher or lower of two results be used?
    pub advantage: Advantage,
    /// The sum of all modifiers
    pub modifier: isize,
    /// The raw value shown by the dice, after advantage has been applied
    pub natural_roll: Option<usize>,
    /// The natural_roll + modifier
    pub result: Option<usize>,
}

impl Roll {
    fn roll_once(&self) -> usize {
        (0..self.n)
            .map(|_| fastrand::usize(1..=self.d as usize))
            .sum()
    }

    pub fn roll(&mut self) {
        let natural_roll = match self.advantage {
            Advantage::Disadvantage => min(self.roll_once(), self.roll_once()),
            Advantage::Neutral => self.roll_once(),
            Advantage::Advantage => max(self.roll_once(), self.roll_once()),
        };
        self.natural_roll = Some(natural_roll);

        let sum = natural_roll as isize + self.modifier;
        // Negative rolls are treated as 0
        self.result = Some(sum.try_into().unwrap_or_default());
    }

    /// The raw value shown by the die, after advantage has been applied
    #[allow(dead_code)]
    pub fn natural_roll(&self) -> Option<usize> {
        self.natural_roll
    }

    /// Computes the result of a roll as if `natural_roll` has been rolled
    #[allow(dead_code)]
    pub fn fixed_roll(&mut self, natural_roll: usize) {
        self.natural_roll = Some(natural_roll);
        let sum = natural_roll as isize + self.modifier;
        // Negative rolls are treated as 0
        self.result = Some(sum.try_into().unwrap_or_default());
    }
}
