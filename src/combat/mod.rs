use bevy::ecs::system::Command;
use bevy::prelude::*;
use bevy::utils::HashMap;

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
        app.init_resource::<Schedules>()
            .add_plugin(actions::ActionPlugin)
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

/// A task that a user can perform during combat
///
/// Each variant corresponds to a [`Schedule`] in [`Schedules`].
/// Flows can be run using [`Commands`], taking effect at the end of the current stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
#[allow(dead_code)]
enum Flow {
    SelectAction,
    SelectTarget,
    RollAttack,
    ResolveAttack,
}

/// Contains various workflows that can be run on the [`World`]
///
/// Stores various [`Schedule`]s, keyed by a [`Flow`] variant
#[derive(Default)]
struct Schedules {
    schedules: HashMap<Flow, Schedule>,
}

impl Schedules {
    fn run(&mut self, flow: Flow, world: &mut World) {
        let schedule = self.schedules.get_mut(&flow).unwrap();
        schedule.run(world);
    }

    fn add_stage_as_flow(&mut self, flow: Flow, stage: SystemStage) {
        let mut schedule = Schedule::default();
        schedule.add_stage(flow, stage);
        self.schedules.insert(flow, schedule);
    }
}

impl Command for Flow {
    fn write(self, world: &mut World) {
        world.resource_scope(|world, mut schedules: Mut<Schedules>| {
            schedules.run(self, world);
        });
    }
}
