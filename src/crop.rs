use bevy::prelude::*;

use crate::player::ItemType;

// idea
// spawn the prepared dirt 
// select the type and then the timer start

#[derive(Debug, Component)]
pub enum CropType {
    Potato
}

#[derive(Debug, Component)]
pub struct PreparedDirtData {
    pub item_type: ItemType,
    pub crop_type: CropType,
    pub growth_state: i32,
    pub growth_active: bool,
    pub growth_complete: bool,
    pub growth_state_timer: Timer,
    pub worker_assigned_bool: bool,
    pub worker_assigned_entity: Entity
}

#[derive(Bundle)]
pub struct DirtBundle {
    pub tf: Transform,
    pub spr: Sprite,
    pub data: PreparedDirtData
}

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