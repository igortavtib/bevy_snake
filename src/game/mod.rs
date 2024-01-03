use bevy::prelude::*;

use self::{systems::setup, snake::SnakePlugin};

mod systems;
mod snake;
mod consts;
mod components;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SnakePlugin).add_systems(Startup, setup);
    }
}
