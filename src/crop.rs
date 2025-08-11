use bevy::prelude::*;

use crate::buildings::PreparedDirtData;

// idea
// change the type and start the timer

pub struct MyCropPlugin;

impl Plugin for MyCropPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, crop_growth_manager);
        app.add_systems(Update, log_crop_data);
    }
}

// iter all the crops available and check if they area active
fn crop_growth_manager(
    time: Res<Time>,
    mut crops: Query<&mut PreparedDirtData, With<PreparedDirtData>>
) {
    for mut crop in &mut crops {
        if crop.growth_active {
            crop.growth_state_timer.tick(time.delta());
            if crop.growth_state_timer.just_finished() { crop.growth_state += 1; }
        }
    }
}

fn log_crop_data(
    input: Res<ButtonInput<KeyCode>>,
    crops: Query<&PreparedDirtData>
) {
    if input.just_pressed(KeyCode::KeyO) {
        for crop in &crops {
            println!("crop: {:?}", crop);
        }
    }
}