use attack::check_attacks;
use bevy::app::{AppBuilder, Plugin};
use bevy::ecs::{Entity, IntoSystem};
use derive_more::{Deref, DerefMut};

/// Events are created whenever actions occur, and are hooked into
/// by systems that power our affixes using a custom scheduler that only recomputes
/// when something relevant has changed.
///
/// The turn-based nature of the game should be handled using parallel game loops.
pub mod actions;
pub mod attack;
pub mod conditions;
pub mod core_actions;
pub mod damage;
pub mod forced_movement;
pub mod height_tiers;
pub mod movement;
pub mod tiles;
pub mod time;
pub mod visibility_cover;

pub struct CombatPlugin {}

/// Marker component for entity-events that are currently being processed
#[allow(dead_code)]
pub struct Active;
#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq)]
pub struct Attacker(Entity);
#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq)]
pub struct Defender(Entity);

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(check_attacks.system());
    }
}
