use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use self::systems::spawn_apple;

pub struct ApplePlugin;

pub mod components;
mod systems;

impl Plugin for ApplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_apple.run_if(on_timer(Duration::from_secs_f32(1.))));
    }
}
