use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn((
            Name::new("GameCamera"),
            Camera2dBundle::default(),
            ));
}
