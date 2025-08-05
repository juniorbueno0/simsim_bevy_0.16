use bevy::{math::NormedVectorSpace, prelude::*};

use crate::{buildings::{BuildingType, Buildings}, player::Item};

#[derive(Debug, Resource)]
pub struct WorkerAmount { total: i32 }

#[derive(Component)]
struct WorkerData {
    coins: i32,
    target_coin: Option<Entity>,
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
        app.add_systems(Update, (worker_amount_update, get_nearest_coin_entity));
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
        data: WorkerData { coins: 0, target_coin: Option::None, house: (0,0) }
    });
}

fn worker_amount_update(
    buildings: Res<Buildings>,
    mut workers: ResMut<WorkerAmount>
) {
    let houses = buildings.data.iter().filter(|d|d.1 == BuildingType::House).count();
    workers.total = (houses * 2) as i32; // change later when houses can be upgraded
}

// patrol function

// get the closest coin entity
fn get_nearest_coin_entity(mut workers: Query<(&Transform, &mut WorkerData), With<WorkerData>>, coins: Query<(&Transform, Entity), With<Item>>) {
    let distance: f32 = 4.;

    if let Some(mut worker) = workers.iter_mut().find(|w|w.1.coins <= 0) {
        if let Some(coin) = coins.iter().find(|c|(c.0.translation.x - worker.0.translation.x).norm() < distance) {
            // println!("{:?}", coin.1);
            worker.1.target_coin = Some(coin.1);

        }
        println!("worker: {:?}", worker.1.target_coin);
    }
}

//use the target coin to get close and store it at the worker inventory