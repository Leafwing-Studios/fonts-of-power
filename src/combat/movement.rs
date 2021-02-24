#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Distance(u8);

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
