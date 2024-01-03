use bevy::prelude::*;
use rand::Rng;

use crate::game::{components::Position, snake::components::{SnakeHead, SnakeSegment}, consts::{GRID_SIZE, GRID_SQUARE_SIZE}};

use super::components::Apple;

pub fn spawn_apple(mut commands: Commands, snake_query: Query<&Position, Or<(With<SnakeHead>, With<SnakeSegment>)>>) {
    
    let mut rng = rand::thread_rng();

    let apple_position: Position = Position {
        x: rng.gen_range(-GRID_SIZE..GRID_SIZE),
        y: rng.gen_range(-GRID_SIZE..GRID_SIZE)
    };

    let mut valid_position = true;

    for position in snake_query.iter() {
        if apple_position == *position {
            valid_position = false;
        }
    }

    if valid_position {
        commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        ..default()
                    },
                    transform: Transform {
                        scale: Vec3::splat(GRID_SQUARE_SIZE),
                        translation: Vec3::new(
                                GRID_SQUARE_SIZE * apple_position.x as f32,
                                GRID_SQUARE_SIZE * apple_position.y as f32,
                                0.0,
                            ),
                        ..default()
                    },
                    ..default()
                },
                Apple,
                apple_position,
            ));
    }
}
