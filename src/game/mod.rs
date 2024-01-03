use bevy::prelude::*;

use self::{systems::setup, snake::SnakePlugin, apple::ApplePlugin};

mod systems;
mod snake;
mod consts;
mod components;
mod apple;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SnakePlugin, ApplePlugin)).add_systems(Startup, setup);
    }
}
