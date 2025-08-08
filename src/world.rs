use bevy::prelude::*;

use crate::{buildings::HouseData, worker::WorkerData};

const HOUR: f32 = 5.0;

#[derive(Debug, Component)]
enum Meridiem {
    AM,
    PM
}

#[derive(Debug, Resource)]
pub struct WorldSettings {
    day_timer: Timer,
    actual_hour: f32,
    meridiem: Meridiem
}

pub struct MyWorldPlugin;

impl Plugin for MyWorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldSettings { day_timer: Timer::from_seconds(HOUR, TimerMode::Repeating), actual_hour: 0.0,  meridiem: Meridiem::AM });
   
        app.add_systems(Update, day_timer_tick);
        app.add_systems(Update, clean_scene);

    }
}

fn day_timer_tick(time: Res<Time>, mut day: ResMut<WorldSettings>) {
    day.day_timer.tick(time.delta());

    if day.day_timer.just_finished() { 
        day.actual_hour += 1.0;
        if day.actual_hour > 12.0 {
            day.actual_hour = 1.0;
            
            day.meridiem = match day.meridiem {
                Meridiem::AM => { Meridiem::PM },
                Meridiem::PM => { Meridiem::AM }
            }
        }
        // println!("{:?} {:?}", day.actual_hour, day.meridiem);
    }
}

fn clean_scene(mut cmm: Commands, input: Res<ButtonInput<KeyCode>>, w: Query<Entity, With<WorkerData>>, h: Query<Entity, With<HouseData>>) {
    if input.just_pressed(KeyCode::KeyR) {
        for worker in w {
            cmm.entity(worker).despawn();
        }

        for house in h {
            cmm.entity(house).despawn();
        }
    }
}