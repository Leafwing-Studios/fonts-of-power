use attack::{apply_crits, check_attacks, dispatch_attacks, prepare_attacks, roll_attacks};
use bevy::app::{AppBuilder, CoreStage, Plugin};
use bevy::ecs::{
    Entity, IntoSystem, ParallelSystemDescriptorCoercion, StageLabel, SystemLabel, SystemStage,
};
use conditions::{apply_afflictions, apply_ailments};
use damage::{apply_damage, apply_resistances, roll_damage};
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

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum AttackStage {
    Setup,
    Resolution,
}

use AttackSystem::*;

pub struct AttackPlugin {}
impl Plugin for AttackPlugin {
    fn build(&self, app: &mut AppBuilder) {
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
        .add_system_to_stage(AttackStage::Setup, roll_attacks.system().label(RollAttacks))
        .add_system_to_stage(AttackStage::Setup, roll_damage.system().label(RollDamage))
        .add_system_to_stage(
            AttackStage::Setup,
            dispatch_attacks
                .system()
                .label(DispatchAttacks)
                .after(RollDamage),
        )
        .add_system_to_stage(
            AttackStage::Setup,
            prepare_attacks
                .system()
                .label(PrepareAttacks)
                .after(PrepareAttacks),
        )
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
/// Marker component for entity-events that are currently being processed
#[allow(dead_code)]
pub struct Active;
#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq)]
pub struct Attacker(Entity);
#[derive(Clone, Copy, Deref, DerefMut, PartialEq, Eq)]
pub struct Defender(Entity);
