use bevy::{math::NormedVectorSpace, prelude::*};

use crate::{buildings::{BuildingType, Buildings, HouseData}, player::{CoinsSpawned, Item}};

#[derive(Debug, Resource)]
pub struct WorkerAmount { total: i32 }

#[derive(Debug, Component)]
pub struct WorkerCollectable;

#[derive(Component)]
struct WorkerData {
    coins: i32,
    house_pos: (i32,i32),
    target_coin_entity: Option<Entity>,
    target_coin_pos: Option<Vec3>,
    target_coin_dir: Option<Vec3>,
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
        // app.add_systems(Update, (wwwz, wwwy));
        app.add_systems(Update, (wwwz, wwwy, wwwx, wwww));
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
        data: WorkerData { coins: 0, target_coin_entity: Option::None, target_coin_pos: Option::None, target_coin_dir: Option::None, house_pos: (0,0) }
    });

    cmm.spawn(WorkerBundle {
        spr: Sprite {
                color: Color::srgb(1., 0.4, 0.4),
                custom_size: Some(Vec2 { x: 1., y: 1. }),
                ..default()
            },
        tf: Transform::from_xyz(12.,12., 2.),
        data: WorkerData { coins: 0, target_coin_entity: Option::None, target_coin_pos: Option::None, target_coin_dir: Option::None, house_pos: (0,0) }
    });

    cmm.spawn(WorkerBundle {
        spr: Sprite {
                color: Color::srgb(1., 0.4, 0.4),
                custom_size: Some(Vec2 { x: 1., y: 1. }),
                ..default()
            },
        tf: Transform::from_xyz(4.,12., 2.),
        data: WorkerData { coins: 0, target_coin_entity: Option::None, target_coin_pos: Option::None, target_coin_dir: Option::None, house_pos: (0,0) }
    });

    cmm.spawn(WorkerBundle {
        spr: Sprite {
                color: Color::srgb(1., 0.4, 0.4),
                custom_size: Some(Vec2 { x: 1., y: 1. }),
                ..default()
            },
        tf: Transform::from_xyz(12.,4., 2.),
        data: WorkerData { coins: 0, target_coin_entity: Option::None, target_coin_pos: Option::None, target_coin_dir: Option::None, house_pos: (0,0) }
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

fn wwwz(mut workers: Query<(&mut Transform, &mut WorkerData,), (With<WorkerData>, Without<WorkerCollectable>)>, coins: Query<(&Transform, Entity, &Item), (With<WorkerCollectable>, Without<WorkerData>)>) {
    let distance: f32 = 8.;

    // if let Some(mut w) = workers.iter_mut().find(|w|(w.1.target_coin_pos == Option::None) && (w.1.coins <= 0)) {
    //     if let Some(coin) = coins.iter().find(|c| (c.0.translation.x - w.0.translation.x).norm() < distance && (c.0.translation.y - w.0.translation.y).norm() < distance) {
    //         let dir = (w.0.translation - coin.0.translation).normalize_or_zero();
    //         w.1.target_coin_dir = Some(dir);
    //         w.1.target_coin_pos = Some(coin.0.translation);
    //         w.1.target_coin_entity = Some(coin.1);
    //     }
    // }

    for (tf, mut data) in &mut workers {

        if let Some(coin) = coins.iter().find(|c| (c.0.translation.x - tf.translation.x).norm() < distance && (c.0.translation.y - tf.translation.y).norm() < distance && data.coins < 1) {
            let dir = (tf.translation - coin.0.translation).normalize_or_zero();
            data.target_coin_dir = Some(dir);
            data.target_coin_pos = Some(coin.0.translation);
            data.target_coin_entity = Some(coin.1);
        }
    }
}

fn wwwy(time: Res<Time>, mut cmm: Commands, mut coins_spawned: ResMut<CoinsSpawned>, mut workers: Query<(&mut Transform, &mut WorkerData), (With<WorkerData>, Without<WorkerCollectable>)>, coins: Query<Entity, (With<WorkerCollectable>, Without<WorkerData>)>) {
    for (mut worker_tf,mut worker_data) in &mut workers {
        if worker_data.target_coin_dir == Option::None || worker_data.target_coin_pos == Option::None || worker_data.target_coin_entity == Option::None { continue; }

        let speed = 1.0;
        let offset = 0.1;
        let worker_pos = worker_data.target_coin_pos.unwrap();
        let worker_dir = worker_data.target_coin_dir.unwrap();

        let reset_worker_coin_data = | worker: &mut WorkerData | {
            worker.target_coin_dir = Option::None;
            worker.target_coin_pos = Option::None;
            worker.target_coin_entity = Option::None;
        };

        if worker_tf.translation.x > (worker_pos.x - offset) && worker_tf.translation.x < (worker_pos.x + offset) {
            worker_data.coins += 1;
            println!(" collected a coin ");
            cmm.entity(worker_data.target_coin_entity.unwrap()).despawn();
            coins_spawned.positions.remove(&(worker_tf.translation.x as i32, worker_tf.translation.y as i32)); // not working properly

            reset_worker_coin_data(&mut worker_data);
        }else { worker_tf.translation -= worker_dir * speed * time.delta_secs(); }
        
        if let Some(_) = coins.iter().find(|c|worker_data.target_coin_entity != Option::None && *c == worker_data.target_coin_entity.unwrap()) { }else { // if coin despawn then worker stops moving
            reset_worker_coin_data(&mut worker_data);
        }
    }
}

// assign it to an available home
fn wwwx(mut houses: Query<(&mut HouseData, &Transform)>, mut workers: Query<(&Transform, &mut WorkerData, Entity), (With<WorkerData>, Without<WorkerCollectable>)>) {
    if let Some(mut house) = houses.iter_mut().find(|(h, _)| h.building_type == BuildingType::House && h.assigned_workers.len() < h.max_capacity as usize ) {
        if let Some(mut worker) = workers.iter_mut().find(|w|w.1.coins == 1 && w.1.house_pos == (0,0)) {
            worker.1.house_pos = (house.1.translation.x as i32,house.1.translation.y as i32);
            house.0.assigned_workers.insert(worker.2);
            println!("assigned: {:?}", worker.2);
        }
    } else { return; }
}

// the worker walks to his house
fn wwww(time: Res<Time>, mut workers: Query<(&mut Transform, &WorkerData, Entity), (With<WorkerData>, Without<WorkerCollectable>)>) {

    for mut w in &mut workers { // change it to filter thern iter
        if w.1.coins == 1 && w.1.house_pos != (0,0) {
            let dir = w.0.translation - Vec3::new(w.1.house_pos.0 as f32,w.1.house_pos.1 as f32, 1.);
            w.0.translation -= dir * time.delta_secs();
        }
    }
}