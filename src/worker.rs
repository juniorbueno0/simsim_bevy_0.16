use bevy::{math::NormedVectorSpace, prelude::*};

use crate::{buildings::{BuildingType, Buildings}, player::{CoinsSpawned, Item}};

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
        app.add_systems(Update, (wwwz, wwwy));
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

fn wwwz(mut workers: Query<(&mut Transform, &mut WorkerData, Entity), (With<WorkerData>, Without<WorkerCollectable>)>, coins: Query<(&Transform, Entity, &Item), (With<WorkerCollectable>, Without<WorkerData>)>) {
    let distance: f32 = 8.;

    // if let Some(worker) = 

    for (tf, mut data, ent) in &mut workers {
        if let Some(coin) = coins.iter().find(|c| (c.0.translation.x - tf.translation.x).norm() < distance && (c.0.translation.y - tf.translation.y).norm() < distance) {
            let dir = (tf.translation - coin.0.translation).normalize_or_zero();
            data.target_coin_dir = Some(dir);
            data.target_coin_pos = Some(coin.0.translation);
            data.target_coin_entity = Some(coin.1);
        }
    }
}

fn wwwy(time: Res<Time>, mut cmm: Commands, mut coins_spawned: ResMut<CoinsSpawned>, mut workers: Query<(&mut Transform, &mut WorkerData), (With<WorkerData>, Without<WorkerCollectable>)>, coins: Query<Entity, (With<WorkerCollectable>, Without<WorkerData>)>) {
    for (mut worker_tf,mut worker_data) in &mut workers {
        if worker_data.target_coin_dir == Option::None || worker_data.target_coin_pos == Option::None { continue; }

        let speed = 1.0;
        let offset = 0.1;
        let worker_pos = worker_data.target_coin_pos.unwrap();
        let worker_dir = worker_data.target_coin_dir.unwrap();

        if worker_tf.translation.x > (worker_pos.x - offset) && worker_tf.translation.x < (worker_pos.x + offset) {
            worker_data.coins += 1;
            cmm.entity(worker_data.target_coin_entity.unwrap()).despawn();
            coins_spawned.positions.remove(&(worker_tf.translation.x as i32, worker_tf.translation.y as i32)); // not working properly
        }else { worker_tf.translation -= worker_dir * speed * time.delta_secs(); }
        
        if let Some(_) = coins.iter().find(|c|worker_data.target_coin_entity != Option::None && *c == worker_data.target_coin_entity.unwrap()) { }else { // if coin despawn then worker stops moving
            worker_data.target_coin_dir = Option::None;
            worker_data.target_coin_pos = Option::None;
            worker_data.target_coin_entity = Option::None;
        }
    }
}