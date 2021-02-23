use crate::combat::{
    conditions::{AfflictionEvent, AilmentEvent, StatusEvent},
    damage::DamageEvent,
    forced_movement::ForcedMovementEvent,
};
use crate::core::dice::{roll, Advantage, DieSize};
use bevy::app::{EventReader, Events};
use bevy::ecs::{Entity, Query, ResMut};

/// Marker component for Attack entities, which store the information about an attack's effects
#[allow(dead_code)]
pub struct Attack;
#[allow(dead_code)]
pub struct AttackEvent {
    attacker: Entity,
    defender: Entity,
    attack: Entity,
    attack_bonus: i8,
    defense: i8,
    advantage: Advantage,
}

#[allow(dead_code)]
pub struct AttackLandedEvent {
    attacker: Entity,
    defender: Entity,
    attack: Entity,
}

pub fn check_attacks(
    mut attacks: EventReader<AttackEvent>,
    mut attacks_landed: ResMut<Events<AttackLandedEvent>>,
) {
    for attack in attacks.iter() {
        if roll(1, DieSize::d20, attack.advantage) as i8 + attack.attack_bonus >= attack.defense {
            attacks_landed.send(AttackLandedEvent {
                attacker: attack.attacker,
                defender: attack.defender,
                attack: attack.attack,
            })
        }
    }
}

// TODO: use indexes to look up data
// TODO: complete logic
#[allow(dead_code, unused_variables, unused_mut)]
pub fn process_attacks(
    mut attacks_landed: EventReader<AttackLandedEvent>,
    attack_data: Query<&Attack>,
    mut damage: ResMut<Events<DamageEvent>>,
    mut ailments: ResMut<Events<AilmentEvent>>,
    mut afflictions: ResMut<Events<AfflictionEvent>>,
    mut statuses: ResMut<Events<StatusEvent>>,
    mut forced_movement: ResMut<Events<ForcedMovementEvent>>,
) {
    for attack in attacks_landed.iter() {}
}
