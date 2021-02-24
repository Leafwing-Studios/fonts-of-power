/// Actions are based via events
use bevy::ecs::Entity;
use std::marker::PhantomData;

pub trait Action: Sized {
    fn event(actor: Entity, target: Option<Entity>) -> ActionEvent<Self>;
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ActionSpeed {
    Major,
    Minor,
    Reaction,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct ActionEvent<T: Action> {
    pub actor: Entity,
    pub target: Option<Entity>,
    pub action_type: PhantomData<T>,
    pub speed: ActionSpeed,
    pub essence: u8,
}
// TODO: figure out how to combine disparate action events to get list of choices
// pub struct ActionChoices(Vec<ActionEvent<Box<dyn Action>>>);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Defend;
impl Action for Defend {
    fn event(actor: Entity, _target: Option<Entity>) -> ActionEvent<Self> {
        ActionEvent::<Self> {
            actor: actor,
            target: None,
            action_type: PhantomData::<Self>::default(),
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
            action_type: PhantomData::<Self>::default(),
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
            action_type: PhantomData::<Self>::default(),
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
            action_type: PhantomData::<Self>::default(),
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
            action_type: PhantomData::<Self>::default(),
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
            action_type: PhantomData::<Self>::default(),
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
