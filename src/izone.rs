use bevy::{
    prelude::*,
    sprite::collide_aabb::collide,
};

#[derive(Component)]
pub struct IZone(i32); 

pub fn setup(mut commands: Commands) {
    let are_boxes_visible = true;
    let mut zone_id: i32 = 0;
    spawn_zone(&mut commands, (200.0, 200.0), (0.0, 0.0), are_boxes_visible, &mut zone_id);
    spawn_zone(&mut commands, (100.0, 200.0), (300.0, 50.0), are_boxes_visible, &mut zone_id);
}

fn spawn_zone(commands: &mut Commands, zonesize: (f32, f32), pos: (f32, f32), is_visible: bool, zone_id: &mut i32) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::ORANGE,
            ..default()
        },
        transform: Transform {
            scale: Vec3::new(zonesize.0, zonesize.1, 0.0),
            translation: Vec3::new(pos.0, pos.1, 0.0),
            ..default()
        },
        visibility: Visibility { is_visible },
        ..default()
    }).insert(IZone(*zone_id));
    *zone_id += 1;
}

pub fn check_zone(query: Query<&Transform, With<crate::player::Player>>,
    izone_query: &mut Query<((&Transform, &mut Sprite, &IZone), With<IZone>), Without<crate::player::Player>>) -> Option<i32> {
    for mut izone in izone_query.iter_mut() {
        let collision = collide(
            izone.0.0.translation,
            izone.0.0.scale.truncate(),
            query.single().translation,
            query.single().scale.truncate(),
        );

        return if collision.is_some() {
            izone.0.1.color = Color::RED;
            //info!("Collision with hitbox number {}", izone.0.2.0);
            Some(izone.0.2.0)
        } else {
            izone.0.1.color = Color::ORANGE;
            None
        }
    }
    None
}