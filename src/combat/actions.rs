use crate::combat::tiles::{Distance, Shape};
use bevy::ecs::{Bundle, Entity};
use derive_more::{Deref, DerefMut};
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
/// - Target
/// - ValidTargets
/// - TargetArity
/// - Range
/// - RangeCategory
/// - AreaOfEffect

// TODO: add archetype invariants

/// Fundamental action component that declares that an entity is an Action
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Action {
    name: String,
    description: String,
}

/// Marker component for actions that are currently being processed
#[allow(dead_code)]
pub struct LiveAction;

/// The actions available to a specific actor
#[derive(Clone, Deref, DerefMut, PartialEq, Eq)]
pub struct ActionEvents(Vec<Entity>);

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ActionSpeed {
    Major,
    Minor,
    Reaction,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ValidTargets {
    Creature,
    Tile,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TargetArity {
    Single,
    Multi,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum RangeCategory {
    Melee,
    Ranged,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Deref, DerefMut)]
pub struct Range(Distance);

#[allow(dead_code)]
pub struct AreaOfEffect(Shape);

// TODO: make archetype bundles for each core action
/// Core action entity marker: these entities should never be changed at run time
#[allow(dead_code)]
pub struct CoreAction;

/*
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Defend;
impl Action for Defend {
    fn event(actor: Entity, target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: None,
            action_type: PhantomData,
            speed: ActionSpeed::Major,
            essence: 0,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Grapple;
impl Action for Grapple {
    fn event(actor: Entity, target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: Some(target.unwrap()),
            action_type: PhantomData,
            speed: ActionSpeed::Major,
            essence: 0,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct ReverseGrapple;
impl Action for ReverseGrapple {
    fn event(actor: Entity, target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: Some(target.unwrap()),
            action_type: PhantomData,
            speed: ActionSpeed::Major,
            essence: 0,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Strike;
impl Action for Strike {
    fn event(actor: Entity, target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: Some(target.unwrap()),
            action_type: PhantomData,
            speed: ActionSpeed::Major,
            essence: 0,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Activate;
impl Action for Activate {
    fn event(actor: Entity, target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: target,
            action_type: PhantomData,
            speed: ActionSpeed::Minor,
            essence: 0,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct BreakGrapple;
impl Action for BreakGrapple {
    fn event(actor: Entity, target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: Some(target.unwrap()),
            action_type: PhantomData,
            speed: ActionSpeed::Minor,
            essence: 0,
        }
    }
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Dash;
impl Action for Dash {
    fn event(actor: Entity, _target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: None,
            action_type: PhantomData::<Self>::default(),
            speed: ActionSpeed::Minor,
            essence: 0,
        }
    }
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct EssenceTap;
impl Action for EssenceTap {
    fn event(actor: Entity, _target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: None,
            action_type: PhantomData::<Self>::default(),
            speed: ActionSpeed::Minor,
            essence: 0,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Guard;
impl Action for Guard {
    fn event(actor: Entity, target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: Some(target.unwrap()),
            action_type: PhantomData::<Self>::default(),
            speed: ActionSpeed::Minor,
            essence: 0,
        }
    }
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Hide;
impl Action for Hide {
    fn event(actor: Entity, _target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: None,
            action_type: PhantomData::<Self>::default(),
            speed: ActionSpeed::Minor,
            essence: 0,
        }
    }
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Interact;
impl Action for Interact {
    fn event(actor: Entity, target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: Some(target.unwrap()),
            action_type: PhantomData::<Self>::default(),
            speed: ActionSpeed::Minor,
            essence: 0,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Scan;
impl Action for Scan {
    fn event(actor: Entity, target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: Some(target.unwrap()),
            action_type: PhantomData::<Self>::default(),
            speed: ActionSpeed::Minor,
            essence: 0,
        }
    }
}
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Shove;
impl Action for Shove {
    fn event(actor: Entity, target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: Some(target.unwrap()),
            action_type: PhantomData::<Self>::default(),
            speed: ActionSpeed::Minor,
            essence: 0,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Swap;
impl Action for Swap {
    fn event(actor: Entity, _target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: None,
            action_type: PhantomData::<Self>::default(),
            speed: ActionSpeed::Minor,
            essence: 0,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Treat;
impl Action for Treat {
    fn event(actor: Entity, target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: Some(target.unwrap()),
            action_type: PhantomData::<Self>::default(),
            speed: ActionSpeed::Minor,
            essence: 0,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct AoO;
impl Action for AoO {
    fn event(actor: Entity, target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: Some(target.unwrap()),
            action_type: PhantomData::<Self>::default(),
            speed: ActionSpeed::Reaction,
            essence: 0,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Block;
impl Action for Block {
    fn event(actor: Entity, target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: Some(target.unwrap()),
            action_type: PhantomData::<Self>::default(),
            speed: ActionSpeed::Reaction,
            essence: 0,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Dodge;
impl Action for Dodge {
    fn event(actor: Entity, target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: Some(target.unwrap()),
            action_type: PhantomData::<Self>::default(),
            speed: ActionSpeed::Reaction,
            essence: 0,
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Track;
impl Action for Track {
    fn event(actor: Entity, target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: Some(target.unwrap()),
            action_type: PhantomData::<Self>::default(),
            speed: ActionSpeed::Reaction,
            essence: 0,
        }
    }
}
*/
