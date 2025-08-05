use bevy::prelude::*;

use crate::buildings::{BuildingType, Buildings};

#[derive(Debug, Resource)]
pub struct WorkerAmount { total: i32 }

#[derive(Component)]
struct WorkerData {
    coins: i32,
    house: (i32,i32)
}

#[derive(Bundle)]
struct WorkerBundle {
    spr: Sprite,
    tf: Transform,
    data: WorkerData
}

pub struct MyWorkerPlugin;

impl Plugin for MyWorkerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorkerAmount { total: 0 });
        app.add_systems(Startup, setup);
        app.add_systems(Update, worker_amount_update);
    }
}

fn setup(mut cmm: Commands) {
    cmm.spawn(WorkerBundle {
        spr: Sprite {
                color: Color::srgb(1., 0.4, 0.4),
                custom_size: Some(Vec2 { x: 1., y: 1. }),
                ..default()
            },
        tf: Transform::from_xyz(4.,4., 2.),
        data: WorkerData { coins: 0, house: (0,0) }
    });
}

fn worker_amount_update(
    buildings: Res<Buildings>,
    mut workers: ResMut<WorkerAmount>
) {
    let houses = buildings.data.iter().filter(|d|d.1 == BuildingType::House).count();
    workers.total = (houses * 2) as i32; // change later when houses can be upgraded
}