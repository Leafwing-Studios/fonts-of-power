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
}

impl Roll {
    fn roll_once(n: i32, d: DieSize) -> i32 {
        let mut rng = rand::thread_rng();

        (0..n).map(|_| rng.gen_range(1..=d as i32)).sum()
    }

    pub fn roll(self) -> i32 {
        match self.advantage {
            Advantage::Disadvantage => {
                min(
                    Roll::roll_once(self.n, self.d),
                    Roll::roll_once(self.n, self.d),
                ) + self.modifier
            }
            Advantage::Neutral => Roll::roll_once(self.n, self.d) + self.modifier,
            Advantage::Advantage => {
                max(
                    Roll::roll_once(self.n, self.d),
                    Roll::roll_once(self.n, self.d),
                ) + self.modifier
            }
        }
    }
}
