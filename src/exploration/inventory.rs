/// Gear, currencies, consumables and adventuring gear
use bevy::prelude::Entity;
use std::collections::HashSet;

/// Component that tracks the items in the inventory
#[allow(dead_code)]
pub struct Inventory(HashSet<Entity>);

/// Number of essence crystals
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct EssenceCrystals(i32);
/// Marker component for key items
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct KeyItem;

/// Marker component for adventuring gear
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct AdventuringGear;

/// Marker component for equippable gear
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Gear;

/// Marker component for consumables
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Consumable;
