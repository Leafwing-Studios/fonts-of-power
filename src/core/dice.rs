use bevy::math::i32;
use derive_more::{Deref, DerefMut};
use rand::Rng;
use std::cmp::{max, min};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Advantage {
    Disadvantage,
    Neutral,
    Advantage,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Deref, DerefMut)]
pub struct NumDice(pub i8);

impl From<i8> for NumDice {
    fn from(n: i8) -> Self {
        NumDice(n)
    }
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

impl From<i8> for DieSize {
    fn from(d: i8) -> Self {
        match d {
            4 => DieSize::d4,
            6 => DieSize::d6,
            8 => DieSize::d8,
            10 => DieSize::d10,
            12 => DieSize::d12,
            20 => DieSize::d20,
            100 => DieSize::d100,
            _ => panic!("Invalid die size"),
        }
    }
}

impl From<DieSize> for i8 {
    #[allow(dead_code)]
    fn from(d: DieSize) -> i8 {
        use DieSize::*;
        match d {
            d4 => 4,
            d6 => 6,
            d8 => 8,
            d10 => 10,
            d12 => 12,
            d20 => 20,
            d100 => 100,
        }
    }
}

impl From<DieSize> for i32 {
    fn from(d: DieSize) -> i32 {
        use DieSize::*;
        match d {
            d4 => 4,
            d6 => 6,
            d8 => 8,
            d10 => 10,
            d12 => 12,
            d20 => 20,
            d100 => 100,
        }
    }
}

pub fn roll_once(n: NumDice, d: DieSize) -> i32 {
    let mut rng = rand::thread_rng();

    (0..*n).map(|_| rng.gen_range(1..=d as i32)).sum()
}

pub fn roll(n: NumDice, d: DieSize, advantage: Advantage) -> i32 {
    match advantage {
        Advantage::Disadvantage => min(roll_once(n, d), roll_once(n, d)),
        Advantage::Neutral => roll_once(n, d),
        Advantage::Advantage => max(roll_once(n, d), roll_once(n, d)),
    }
}
