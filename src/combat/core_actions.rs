use crate::combat::actions::{
    Action, ActionPoints, Actor, Range, RangeCategory, TargetArity, Targets, ValidTargets,
};
use crate::combat::time::Duration;
use bevy::prelude::{Bundle, Component};

// TODO: derive Default for each core action
/// Core action entity marker: these entities should never be changed at run time
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct CoreAction;

#[derive(Bundle, Clone)]
pub struct MoveAction {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
}

#[derive(Bundle, Clone)]
pub struct ClimbAction {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
}

#[derive(Bundle, Clone)]
pub struct ProneAction {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
}

#[derive(Bundle, Clone)]
pub struct UnproneAction {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
}

#[derive(Bundle, Clone)]
pub struct DefendAction {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
}

#[derive(Bundle, Clone)]
pub struct GrappleAction {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
    target: Targets,
    valid_targets: ValidTargets,
    target_arity: TargetArity,
    range: Range,
    range_category: RangeCategory,
}

#[derive(Bundle, Clone)]
pub struct ReverseGrappleAction {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
    target: Targets,
    valid_targets: ValidTargets,
    target_arity: TargetArity,
    range: Range,
    range_category: RangeCategory,
}

#[derive(Bundle, Clone)]
pub struct Strike {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
    target: Targets,
    valid_targets: ValidTargets,
    target_arity: TargetArity,
    range: Range,
    range_category: RangeCategory,
}

#[derive(Bundle, Clone)]
pub struct Activate {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
}
#[derive(Bundle, Clone)]
pub struct BreakGrapple {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
    target: Targets,
    valid_targets: ValidTargets,
    target_arity: TargetArity,
    range: Range,
    range_category: RangeCategory,
}

#[derive(Bundle, Clone)]
pub struct Dash {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
}

#[derive(Bundle, Clone)]
pub struct EssenceTap {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
}

#[derive(Bundle, Clone)]
pub struct Guard {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
}

#[derive(Bundle, Clone)]
pub struct Hide {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
}

#[derive(Bundle, Clone)]
pub struct Interact {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
    target: Targets,
    valid_targets: ValidTargets,
    target_arity: TargetArity,
    range: Range,
    range_category: RangeCategory,
}

#[derive(Bundle, Clone)]
pub struct Scan {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
}

#[derive(Bundle, Clone)]
pub struct Shove {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
    target: Targets,
    valid_targets: ValidTargets,
    target_arity: TargetArity,
    range: Range,
    range_category: RangeCategory,
}

#[derive(Bundle, Clone)]
pub struct Swap {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
}

#[derive(Bundle, Clone)]
pub struct Treat {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
    target: Targets,
    valid_targets: ValidTargets,
    target_arity: TargetArity,
    range: Range,
    range_category: RangeCategory,
}

#[derive(Bundle, Clone)]
pub struct AoO {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
    target: Targets,
    valid_targets: ValidTargets,
    target_arity: TargetArity,
    range: Range,
    range_category: RangeCategory,
}

#[derive(Bundle, Clone)]
pub struct Block {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
    duration: Duration,
}

#[derive(Bundle, Clone)]
pub struct Dodge {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
}

#[derive(Bundle, Clone)]
pub struct Track {
    core_action: CoreAction,
    action: Action,
    action_points: ActionPoints,
    actor: Actor,
    target: Targets,
    valid_targets: ValidTargets,
    target_arity: TargetArity,
    range: Range,
    range_category: RangeCategory,
}
