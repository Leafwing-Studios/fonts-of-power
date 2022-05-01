use actions::identify_targets;
use attack::{
    apply_crits, apply_efficacy, check_attacks, dispatch_attacks, get_attack_bonuses, get_defenses,
    roll_attacks,
};
use bevy::app::{App, CoreStage, Plugin};
use bevy::prelude::*;
use conditions::{apply_afflictions, apply_ailments};
use damage::{apply_damage, apply_resistances, roll_damage};

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

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum ActionStage {
    Selection,
    Dispatch,
}

pub struct ActionPlugin;
impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(
            AttackStage::Setup,
            ActionStage::Dispatch,
            SystemStage::parallel(),
        )
        .add_stage_before(
            ActionStage::Dispatch,
            ActionStage::Selection,
            SystemStage::parallel(),
        )
        .add_system_to_stage(ActionStage::Dispatch, identify_targets);
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum AttackStage {
    Setup,
    Resolution,
}

use self::damage::LifeLost;

pub struct AttackPlugin;
impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        // Adding stages
        app.add_stage_before(
            CoreStage::Update,
            AttackStage::Resolution,
            SystemStage::parallel(),
        )
        .add_stage_before(
            CoreStage::Update,
            AttackStage::Setup,
            SystemStage::parallel(),
        )
        // AttackStage::Setup
        .add_system_to_stage(AttackStage::Setup, get_attack_bonuses)
        .add_system_to_stage(AttackStage::Setup, roll_attacks.after(get_attack_bonuses))
        .add_system_to_stage(AttackStage::Setup, roll_damage)
        .add_system_to_stage(
            AttackStage::Setup,
            dispatch_attacks.after(roll_damage).after(roll_attacks),
        )
        .add_system_to_stage(AttackStage::Setup, get_defenses.after(dispatch_attacks))
        // AttackStage::Resolution
        .add_system_to_stage(AttackStage::Resolution, check_attacks)
        .add_system_to_stage(
            AttackStage::Resolution,
            apply_crits.after(check_attacks).after(roll_damage),
        )
        .add_system_to_stage(
            AttackStage::Resolution,
            apply_resistances.after(apply_crits),
        )
        .add_system_to_stage(AttackStage::Resolution, apply_efficacy.after(apply_crits))
        .add_system_to_stage(
            AttackStage::Resolution,
            apply_damage.after(apply_resistances).after(apply_efficacy),
        )
        .add_system_to_stage(
            AttackStage::Resolution,
            apply_afflictions.after(apply_efficacy),
        )
        .add_system_to_stage(
            AttackStage::Resolution,
            apply_ailments.after(apply_efficacy),
        );

        // Events
        app.add_event::<LifeLost>();
    }
}

// Shared resources and components for Combat

/// Marker component for entity-events that are currently being processed
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Active;

/// Objects that can be interacted with in combat
#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ObjectKind {
    Creature,
    Inanimate,
    Tile,
}
