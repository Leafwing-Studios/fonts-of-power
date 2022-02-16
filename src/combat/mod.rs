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

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ActionSystem {
    IdentifyTargets,
}

use ActionSystem::*;

pub struct ActionPlugin {}
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
        .add_system_to_stage(
            ActionStage::Dispatch,
            identify_targets.label(IdentifyTargets),
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
    GetAttackBonuses,
    CheckAttacks,
    RollAttacks,
    RollDamage,
    DispatchAttacks,
    GetDefenses,
    ApplyCrits,
    ApplyEfficacy,
    ApplyResistances,
    ApplyDamage,
    ApplyAfflictions,
    ApplyAilments,
}

use AttackSystem::*;

pub struct AttackPlugin {}
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
        .add_system_to_stage(
            AttackStage::Setup,
            get_attack_bonuses.label(GetAttackBonuses),
        )
        .add_system_to_stage(
            AttackStage::Setup,
            roll_attacks.label(RollAttacks).after(GetAttackBonuses),
        )
        .add_system_to_stage(AttackStage::Setup, roll_damage.label(RollDamage))
        .add_system_to_stage(
            AttackStage::Setup,
            dispatch_attacks
                .label(DispatchAttacks)
                .after(RollDamage)
                .after(RollAttacks),
        )
        .add_system_to_stage(
            AttackStage::Setup,
            get_defenses.label(GetDefenses).after(DispatchAttacks),
        )
        // AttackStage::Resolution
        .add_system_to_stage(AttackStage::Resolution, check_attacks.label(CheckAttacks))
        .add_system_to_stage(
            AttackStage::Resolution,
            apply_crits
                .label(ApplyCrits)
                .after(CheckAttacks)
                .after(RollDamage),
        )
        .add_system_to_stage(
            AttackStage::Resolution,
            apply_resistances.label(ApplyResistances).after(ApplyCrits),
        )
        .add_system_to_stage(
            AttackStage::Resolution,
            apply_efficacy.label(ApplyEfficacy).after(ApplyCrits),
        )
        .add_system_to_stage(
            AttackStage::Resolution,
            apply_damage.label(ApplyDamage).after(ApplyResistances),
        )
        .add_system_to_stage(
            AttackStage::Resolution,
            apply_afflictions.label(ApplyAfflictions).after(ApplyCrits),
        )
        .add_system_to_stage(
            AttackStage::Resolution,
            apply_ailments.label(ApplyAilments).after(ApplyCrits),
        );
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
