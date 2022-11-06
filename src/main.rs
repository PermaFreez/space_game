mod walls;
mod player;

use bevy::prelude::*;

use crate::walls::draw_walls;
use crate::player::player_handling;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::TEAL));

    app.add_system(player_handling::main)
        .add_system(draw_walls::main);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        transform: Transform {
            scale: Vec3::new(75.0, 100.0, 0.0),
            ..default()
        },
        sprite: Sprite {
            color: Color::SEA_GREEN,
            ..default()
        },
        ..default()
    }).insert(player_handling::Player);
}