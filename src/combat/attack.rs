use crate::core::dice::Advantage;
use bevy::ecs::Entity;
#[allow(dead_code)]
pub struct AttackEvent {
    attack: i8,
    defense: i8,
    advantage: Advantage,
    attacker: Entity,
    defender: Entity,
}
