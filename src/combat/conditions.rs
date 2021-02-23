use bevy::ecs::Entity;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Condition {
    Affliction(Affliction),
    Ailment(Ailment),
    Status(Status),
}

#[non_exhaustive]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Affliction {
    Aflame,
    Anguish,
    Bleeding,
    Brittle,
    Rage,
    Shock,
    Temperance,
    Unstable,
}

#[non_exhaustive]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Ailment {
    Aloft,
    Banished,
    Blinded,
    Blunted,
    Chilled,
    Crazed,
    Cursed,
    Defenseless,
    Frightened,
    Hapless,
    Pacified,
    Rooted,
    Taunted,
    Withered,
}

#[non_exhaustive]
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Status {
    DeathsDoor,
    Flying,
    Grappled,
    Grappling,
    Hidden,
    Invisible,
    Prone,
    Unconscious,
}

#[allow(dead_code)]
pub struct AfflictionEvent {
    attacker: Entity,
    defender: Entity,
    affliction: Affliction,
    stacks: u8,
}

#[allow(dead_code)]
pub struct AilmentEvent {
    attacker: Entity,
    defender: Entity,
    ailment: Ailment,
    stacks: u8,
}

#[allow(dead_code)]
pub struct StatusEvent {
    attacker: Entity,
    defender: Entity,
    status: Status,
    toggle: bool,
}
