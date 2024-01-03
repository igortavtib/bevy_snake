use bevy::prelude::*;

use self::systems::setup;

mod systems;
pub mod components;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
