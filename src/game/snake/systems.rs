use std::convert;

use bevy::prelude::*;

use crate::{game::{consts::{GRID_SQUARE_SIZE, GRID_SIZE}, components::Position}, WINDOW_WIDTH, WINDOW_HEIGH};

use super::components::{SnakeHead, SnakeSegment, SnakeDirection};

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

pub fn movement_input(mut query: Query<&mut SnakeHead>, keyboard_input: Res<Input<KeyCode>>) {
    let mut head = query.single_mut();

    if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
        head.direction = SnakeDirection::Up;
    }
    if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
        head.direction = SnakeDirection::Down;
    }
    if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
        head.direction = SnakeDirection::Right;
    }
    if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
        head.direction = SnakeDirection::Left;
    }
}

pub fn movement(
    mut query: Query<(&mut SnakeHead, &mut Position), (With<SnakeHead>, Without<SnakeSegment>)>,
    mut segment_query: Query<&mut Position, (With<SnakeSegment>, Without<SnakeHead>)>,
) {
    let (head, mut position) = query.single_mut();

    let mut prev_position = position.clone();

    match head.direction {
        SnakeDirection::Up => {
            position.y += 1;
        },
        SnakeDirection::Down => {
            position.y -= 1;
        },
        SnakeDirection::Right => {
            position.x += 1;
        },
        SnakeDirection::Left => {
            position.x -= 1;
        }
    }

    for mut position in segment_query.iter_mut() {
        let segment_position = position.clone();
        position.x = prev_position.x;
        position.y = prev_position.y;

        prev_position = segment_position;
    }
}

pub fn movement_tranform(mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut tranform) in query.iter_mut() {
        tranform.translation = Vec3::new(
            (WINDOW_WIDTH / GRID_SIZE as f32) * position.x as f32,
            (WINDOW_HEIGH / GRID_SIZE as f32) * position.y as f32,
            0.0,
            )
    }
}
