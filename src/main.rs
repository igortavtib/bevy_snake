use bevy::prelude::*;
use game::GamePlugin;

const WINDOW_WIDTH: f32 = 700.;
const WINDOW_HEIGH: f32 = 700.;

mod game;

fn main() {
    App::new().add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Snake".into(),
                    resolution: (WINDOW_WIDTH, WINDOW_HEIGH).into(),
                    ..default()
                }),
                ..default()
            }).build()
        ).
        add_plugins(GamePlugin)
        .run();
}
