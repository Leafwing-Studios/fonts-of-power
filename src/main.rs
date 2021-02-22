use bevy::prelude::{App, DefaultPlugins};
mod dice;
mod skills;
mod stats;

fn main() {
    App::build().add_plugins(DefaultPlugins).run();
}
