use attack::{check_attacks, AttackEvent};
use bevy::app::{AppBuilder, Plugin};
use bevy::ecs::IntoSystem;

/// Events are created whenever actions occur, and are hooked into
/// by systems that power our affixes using a custom scheduler that only recomputes
/// when something relevant has changed.
///
/// The turn-based nature of the game should be handled using parallel game loops.
pub mod actions;
pub mod attack;
pub mod concentration;
pub mod conditions;
pub mod damage;
pub mod forced_movement;
pub mod height_tiers;
pub mod movement;
pub mod tiles;
pub mod visibility_cover;

pub struct CombatPlugin {}

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AttackEvent>()
            .add_system(check_attacks.system());
    }
}
