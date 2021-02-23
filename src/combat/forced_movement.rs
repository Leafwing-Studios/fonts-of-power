use crate::combat::movement::{Direction, Distance, Tile};
use bevy::ecs::Entity;
use std::fmt::Debug;
#[allow(dead_code)]
pub struct ForcedMovementEvent {
    attacker: Entity,
    defender: Entity,
    forced_movement: ForcedMovement,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct ForcedMovement {
    target: Entity,
    data: ForcedMovementData,
}
#[allow(dead_code)]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum ForcedMovementData {
    Teleportation {
        new_tile: Tile,
    },
    Push {
        source: Tile,
        distance: Distance,
    },
    Pull {
        source: Tile,
        distance: Distance,
    },
    Shove {
        distance: Distance,
        direction: Direction,
    },
    Shift {
        direction: Direction,
    },
}
