mod walls;
mod player;
mod izone;

use bevy::prelude::*;


fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::TEAL));

    app.add_startup_system(setup)
        .add_startup_system(player::setup)
        .add_startup_system(izone::setup);
    
    app.add_system(player::main)
        .add_system(walls::main)
        .add_system(izone::check_zone);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}