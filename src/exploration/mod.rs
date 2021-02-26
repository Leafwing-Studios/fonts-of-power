pub mod downtime;
pub mod inventory;
pub mod scenes;
use bevy::app::{AppBuilder, Plugin};
pub struct ExplorationPlugin {}
impl Plugin for ExplorationPlugin {
    fn build(&self, _app: &mut AppBuilder) {}
}
