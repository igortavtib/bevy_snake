use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use self::systems::{setup, movement_input, movement, movement_tranform};

mod systems;
pub mod components;

pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (
                movement_input,
                movement,
                movement_tranform,
            ).run_if(on_timer(Duration::from_secs_f32(1. / 9.))));
    }
}
