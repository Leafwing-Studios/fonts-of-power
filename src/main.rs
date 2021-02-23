use bevy::prelude::{App, DefaultPlugins};
mod character_options;
mod combat;
mod core;
mod exploration;

fn main() {
    App::build().add_plugins(DefaultPlugins).run();
}
