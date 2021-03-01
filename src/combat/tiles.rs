use crate::combat::attack::Efficacy;
use derive_more::{Deref, DerefMut};
use num_rational::Ratio;
use std::ops::Mul;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Distance {
    pub val: u32,
}

/// Used to scale forced movement effects with efficacy
impl Mul<Efficacy> for Distance {
    type Output = Self;

    fn mul(self, rhs: Efficacy) -> Self {
        let mut new = self.clone();
        let lhs = Ratio::from_integer(self.val as u16);

        new.val = (lhs * rhs.val).to_integer() as u32;
        new
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deref, DerefMut)]
pub struct Area(u16);

// TODO: Revise Position, Direction etc. to be traits, then make generic over Hex and Square
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Position {
    GridPosition { x: i8, y: i8 },
    HexPosition { alpha: i8, beta: i8 },
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Direction {
    Grid(GridDirection),
    Hex(HexDirection),
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum GridDirection {
    North,
    NorthEast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum HexDirection {
    North,
    NorthEast,
    Southeast,
    South,
    Southwest,
    Northwest,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Shape {
    Circle {
        radius: Distance,
        center: Position,
    },
    Square {
        length: Distance,
        bottom_left_corner: Position,
    },
    Rectangle {
        length: Distance,
        width: Distance,
        bottom_left_corner: Position,
    },
    Cone {
        radius: Distance,
        tip: Position,
    },
    Line {
        length: Distance,
        width: Distance,
        start: Position,
    },
}
