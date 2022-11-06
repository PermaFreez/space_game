pub mod draw_walls {
    use bevy::prelude::*;

    const THICKNESS: f32 = 25.0;
    const X: f32 = 1800.0;
    const Y: f32 = 1000.0;
    
    #[derive(Component)]
    pub struct Wall;
    
    pub fn main(mut commands: Commands) {
        commands.spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(X, THICKNESS, 0.0),
                translation: Vec3::new(0.0, (Y - THICKNESS) / 2.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::GRAY,
                ..default()
            },
            ..default()
        }).insert(Wall);

        commands.spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(THICKNESS, Y, 0.0),
                translation: Vec3::new((X - THICKNESS) / 2.0, 0.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::GRAY,
                ..default()
            },
            ..default()
        }).insert(Wall);

        commands.spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(X, THICKNESS, 0.0),
                translation: Vec3::new(0.0, (-Y + THICKNESS) / 2.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::GRAY,
                ..default()
            },
            ..default()
        }).insert(Wall);

        commands.spawn_bundle(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(THICKNESS, Y, 0.0),
                translation: Vec3::new((-X + THICKNESS) / 2.0, 0.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::GRAY,
                ..default()
            },
            ..default()
        }).insert(Wall);
    }
}