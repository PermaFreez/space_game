pub mod player_handling {
    use bevy::{
        prelude::*,
        sprite::collide_aabb::{collide, Collision},
    };

    #[derive(Component)]
    pub struct Player;

    pub fn main(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>,
                    wall_query: Query<(&Transform, With<crate::walls::draw_walls::Wall>), Without<Player>>) {

        if  keyboard_input.pressed(KeyCode::W) && collision(query.single(), &wall_query, "up") {
            query.single_mut().translation.y += 10.0;
        }
        if  keyboard_input.pressed(KeyCode::A) && collision(query.single(), &wall_query, "left") {
            query.single_mut().translation.x -= 10.0;
        }
        if  keyboard_input.pressed(KeyCode::S) && collision(query.single(), &wall_query, "down") {
            query.single_mut().translation.y -= 10.0;
        }
        if  keyboard_input.pressed(KeyCode::D) && collision(query.single(), &wall_query, "right") {
            query.single_mut().translation.x += 10.0;
        }
    }

    fn collision(player_transform: &Transform, wall_query: &Query<(&Transform, With<crate::walls::draw_walls::Wall>), Without<Player>>, side: &str) -> bool {

        let mut left = true;
        let mut up = true;
        let mut right = true;
        let mut down = true;

        for wall in wall_query.iter() {
            let collision = collide(
                wall.0.translation,
                wall.0.scale.truncate(),
                player_transform.translation,
                player_transform.scale.truncate(),
            );

            match collision {
                Some(Collision::Left) => {left = false},
                Some(Collision::Top) => {up = false},
                Some(Collision::Right) => {right = false},
                Some(Collision::Bottom) => {down = false},
                _ => {},
            }
        }

        match side {
            "left" => left,
            "up" => up,
            "right" => right,
            "down" => down,
            _ => true,
        }
    }
}