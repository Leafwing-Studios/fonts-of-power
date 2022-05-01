#![allow(clippy::type_complexity)]
// TODO: remove once https://github.com/bevyengine/bevy/issues/4601 is fixed
#![allow(clippy::forget_non_drop)]

use bevy::{app::App, DefaultPlugins};
mod character_options;
mod combat;
mod core;
mod exploration;
mod tests;

/// Generates a headless vesion of the game, suitable for use in integration tests
pub fn headless_app() -> App {
    let mut app = App::new();

    app.add_plugin(core::CorePlugin)
        .add_plugin(combat::CombatPlugin)
        .add_plugin(exploration::ExplorationPlugin)
        .add_plugin(character_options::CharacterOptionsPlugin);

    app
}

fn main() {
    let mut app = headless_app();

    // Add remaining plugins that will not work in headless tests or on CI
    app.add_plugins(DefaultPlugins).run();
}
