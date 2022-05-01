use bevy::prelude::*;

/// Events are created whenever actions occur, and are hooked into
/// by systems that power our affixes using a custom scheduler that only recomputes
/// when something relevant has changed.
///
/// The turn-based nature of the game should be handled using parallel game loops.
pub mod actions;
pub mod attack;
pub mod conditions;
pub mod core_actions;
pub mod forced_movement;
pub mod height_tiers;
pub mod movement;
pub mod tiles;
pub mod time;
pub mod visibility_cover;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(actions::ActionPlugin)
            .add_plugin(attack::AttackPlugin);
    }
}

/// Marker component for entity-events that are currently being processed
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Active;

/// Objects that can be interacted with in combat
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum ObjectKind {
    Creature,
    Inanimate,
    Tile,
}
