use bevy::prelude::Component;

pub enum SnakeDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Component)]
pub struct SnakeHead {
    pub direction: SnakeDirection,
}

#[derive(Component)]
pub struct SnakeSegment;

impl Default for SnakeHead {
    fn default() -> Self {
        SnakeHead {
            direction: SnakeDirection::Up
        }
    }
}
