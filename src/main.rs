#![allow(dead_code)]
#![allow(clippy::type_complexity)]
// TODO: remove once https://github.com/bevyengine/bevy/issues/4601 is fixed
#![allow(clippy::forget_non_drop)]

use bevy::{app::App, ecs::schedule::ReportExecutionOrderAmbiguities, DefaultPlugins};
mod character_options;
mod combat;
mod core;
mod exploration;

use crate::{
    character_options::CharacterOptionsPlugin,
    combat::{ActionPlugin, AttackPlugin},
    core::CorePlugin,
    exploration::ExplorationPlugin,
};

fn main() {
    App::new()
        .insert_resource(ReportExecutionOrderAmbiguities)
        .add_plugins(DefaultPlugins)
        .add_plugin(CorePlugin)
        .add_plugin(AttackPlugin)
        .add_plugin(ActionPlugin)
        .add_plugin(ExplorationPlugin)
        .add_plugin(CharacterOptionsPlugin)
        .run();
}
