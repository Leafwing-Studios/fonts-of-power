use bevy::ecs::{Command, Commands, Entity, Resources, World};

pub struct CloneEntity;
impl Command for CloneEntity {
    fn write(self: Box<Self>, _world: &mut World, _resources: &mut Resources) {}
}

pub struct CloneEffect;
impl Command for CloneEffect {
    fn write(self: Box<Self>, _world: &mut World, _resources: &mut Resources) {}
}

pub trait EntityCloning {
    fn clone_entity(&mut self, _entity: Entity) {}

    fn clone_effect(&mut self, _entity: Entity, _target: Entity) {}
}

// TODO: insert magic here. See: https://discord.com/channels/691052431525675048/730525730601041940/813570672110469190
impl EntityCloning for Commands {
    fn clone_entity(&mut self, _entity: Entity) {}

    fn clone_effect(&mut self, _entity: Entity, _target: Entity) {}
}
