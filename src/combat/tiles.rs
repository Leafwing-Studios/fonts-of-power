use crate::combat::attack::Efficacy;
use num_rational::Ratio;
use std::ops::Mul;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Distance {
    pub val: usize,
}

/// Used to scale forced movement effects with efficacy
impl Mul<Efficacy> for Distance {
    type Output = Self;

    fn mul(self, rhs: Efficacy) -> Self {
        let mut new = self.clone();
        let lhs = Ratio::from_integer(self.val as usize);

        new.val = (lhs * rhs.val).to_integer() as usize;
        new
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Area {
    tiles: usize,
}

// TODO: Revise Position, Direction etc. to be traits, then make generic over Hex and Square
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Position {
    GridPosition { x: isize, y: isize },
    HexPosition { alpha: isize, beta: isize },
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
