pub mod classes;
pub mod gear;
pub mod species;

use bevy::app::{AppBuilder, Plugin};
pub struct CharacterOptionsPlugin {}
impl Plugin for CharacterOptionsPlugin {
    fn build(&self, _app: &mut AppBuilder) {}
}
