use actions::identify_targets;
use attack::{apply_crits, check_attacks, dispatch_attacks, prepare_attacks, roll_attacks};
use bevy::app::{AppBuilder, CoreStage, Plugin};
use bevy::ecs::{
    IntoSystem, ParallelSystemDescriptorCoercion, StageLabel, SystemLabel, SystemStage,
};
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ActionSystem {
    IdentifyTargets,
}

use ActionSystem::*;

pub struct ActionPlugin {}
impl Plugin for ActionPlugin {
    fn build(&self, app: &mut AppBuilder) {
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
        .add_system_to_stage(
            ActionStage::Dispatch,
            identify_targets.system().label(IdentifyTargets),
        );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum AttackStage {
    Setup,
    Resolution,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum AttackSystem {
    CheckAttacks,
    RollAttacks,
    RollDamage,
    DispatchAttacks,
    PrepareAttacks,
    ApplyCrits,
    ApplyResistances,
    ApplyDamage,
    ApplyAfflictions,
    ApplyAilments,
}

use AttackSystem::*;

pub struct AttackPlugin {}
impl Plugin for AttackPlugin {
    fn build(&self, app: &mut AppBuilder) {
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
        .add_system_to_stage(AttackStage::Setup, roll_attacks.system().label(RollAttacks))
        .add_system_to_stage(AttackStage::Setup, roll_damage.system().label(RollDamage))
        .add_system_to_stage(
            AttackStage::Setup,
            dispatch_attacks
                .system()
                .label(DispatchAttacks)
                .after(RollDamage)
                .after(RollAttacks),
        )
        .add_system_to_stage(
            AttackStage::Setup,
            prepare_attacks
                .system()
                .label(PrepareAttacks)
                .after(DispatchAttacks),
        )
        // AttackStage::Resolution
        .add_system_to_stage(
            AttackStage::Resolution,
            check_attacks.system().label(CheckAttacks),
        )
        .add_system_to_stage(
            AttackStage::Resolution,
            apply_crits
                .system()
                .label(ApplyCrits)
                .after(PrepareAttacks)
                .after(RollDamage),
        )
        .add_system_to_stage(
            AttackStage::Resolution,
            apply_resistances
                .system()
                .label(ApplyResistances)
                .after(ApplyCrits),
        )
        .add_system_to_stage(
            AttackStage::Resolution,
            apply_damage
                .system()
                .label(ApplyDamage)
                .after(ApplyResistances),
        )
        .add_system_to_stage(
            AttackStage::Resolution,
            apply_afflictions
                .system()
                .label(ApplyAfflictions)
                .after(ApplyCrits),
        )
        .add_system_to_stage(
            AttackStage::Resolution,
            apply_ailments
                .system()
                .label(ApplyAilments)
                .after(ApplyCrits),
        );
    }
}

// Shared resources and components for Combat

/// Marker component for entity-events that are currently being processed
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Active;

/// Objects that can be interacted with in combat
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ObjectKind {
    Creature,
    Inanimate,
    Tile,
}
