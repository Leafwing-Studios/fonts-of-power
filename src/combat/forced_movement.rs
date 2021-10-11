use crate::combat::attack::Efficacy;
use crate::combat::tiles::{Direction, Distance, Position};
use bevy::prelude::Component;
use std::fmt::Debug;
use std::ops::Mul;

#[allow(dead_code)]
#[derive(Component, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ForcedMovement {
    Push {
        source: Position,
        distance: Distance,
    },
    Pull {
        source: Position,
        distance: Distance,
    },
    Shove {
        direction: Direction,
        distance: Distance,
    },
    Shift {
        directions: Vec<Direction>,
    },
    Teleport {
        position: Position,
    },
}

impl Mul<Efficacy> for ForcedMovement {
    type Output = Self;

    fn mul(self, rhs: Efficacy) -> Self {
        match self {
            ForcedMovement::Push { source, distance } => ForcedMovement::Push {
                source,
                distance: distance * rhs,
            },
            ForcedMovement::Pull { source, distance } => ForcedMovement::Pull {
                source,
                distance: distance * rhs,
            },
            ForcedMovement::Shove {
                direction,
                distance,
            } => ForcedMovement::Shove {
                direction,
                distance: distance * rhs,
            },
            ForcedMovement::Shift { directions } => ForcedMovement::Shift { directions },
            ForcedMovement::Teleport { position } => ForcedMovement::Teleport { position },
        }
    }
}
