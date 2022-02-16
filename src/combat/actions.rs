use crate::combat::{
    tiles::{Distance, Shape},
    ObjectKind,
};
use bevy::prelude::Component;
use bevy::prelude::Entity;
use std::collections::HashSet;

/// Workflow for actions:
/// 1. Determine which actions are possible for the unit
/// 2. Select one of those actions
/// 3. Emit an event containing that action, specialized by type
/// 4. Hook into that event to actually do things with the action
///
/// Each action is a unique entity, initialized from a prefab that defines the default behavior of that action
///
/// Each action type should have its own Bundle of components
/// The following components are always needed:
/// - Action
/// - ActionSpeed
/// - Actor
///
/// The following components are sometimes needed:
/// - Essence
/// - Targets
/// - ValidTargets
/// - TargetArity
/// - Range
/// - RangeCategory
/// - AreaOfEffect
/// - Duration

// TODO: add archetype invariants

/// Fundamental action component that declares that an entity is an Action
#[derive(Component, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Action {
    name: String,
    description: String,
}

/// The actions available to a specific actor
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionChoices {
    choices: Vec<Entity>,
}

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ActionSpeed {
    Movement,
    Major,
    Minor,
    Reaction,
}

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Actor {
    entity: Entity,
}

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct ValidTargets(HashSet<ObjectKind>);

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum TargetArity {
    Single,
    Multi,
}

#[derive(Component, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum RangeCategory {
    Melee,
    Ranged,
}

#[derive(Component, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Range {
    tiles: Distance,
}

#[derive(Component, Clone, Debug, Hash, PartialEq, Eq)]
pub struct AreaOfEffect {
    shape: Shape,
}

#[derive(Component, Clone, Debug, PartialEq, Eq)]
pub struct Targets {
    pub entities: Vec<Entity>,
}

// TODO: Complete
/// Identifies which targets will be hit by the attack
pub fn identify_targets() {}
