use bevy::prelude::{App, DefaultPlugins};
mod affixes;
mod class;
mod conditions;
mod dice;
mod skills;
mod species;
mod stats;

fn main() {
    App::build().add_plugins(DefaultPlugins).run();
}
