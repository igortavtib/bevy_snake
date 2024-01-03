use bevy::prelude::*;

use crate::game::{consts::GRID_SQUARE_SIZE, components::Position};

use super::components::SnakeHead;

pub fn setup(mut commands: Commands) {
    commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::MIDNIGHT_BLUE,
                    ..default()
                },
                transform: Transform::default().with_scale(Vec3::splat(GRID_SQUARE_SIZE)),
                ..default()
            },
            SnakeHead::default(),
            Position {
                x: 0,
                y: 0
            }
            ));
}
