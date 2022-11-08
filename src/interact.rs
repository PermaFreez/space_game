use bevy::prelude::*;
use crate::izone;

pub fn main(keyboard_input: Res<Input<KeyCode>>, query: Query<&Transform, With<crate::player::Player>>,
    mut izone_query: Query<((&Transform, &mut Sprite, &izone::IZone), With<izone::IZone>), Without<crate::player::Player>>) {

        info!("{}", izone_query.size());
    
    if keyboard_input.just_pressed(KeyCode::E) {
       info!("{:?}", izone::check_zone(query, &mut izone_query));
    }
}