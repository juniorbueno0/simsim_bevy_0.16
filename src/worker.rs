use bevy::{math::NormedVectorSpace, prelude::*};

use crate::{buildings::{BuildingType, Buildings}, player::Item};

#[derive(Debug, Resource)]
pub struct WorkerAmount { total: i32 }

#[derive(Debug, Component)]
pub struct WorkerCollectable;

#[derive(Component)]
struct WorkerData {
    coins: i32,
    house_pos: (i32,i32),
    target_coin_entity: Option<Entity>,
    targer_coin_pos: Option<Vec3>,
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
        data: WorkerData { coins: 0, target_coin_entity: Option::None, targer_coin_pos: Option::None, target_coin_dir: Option::None, house_pos: (0,0) }
    });

    cmm.spawn(WorkerBundle {
        spr: Sprite {
                color: Color::srgb(1., 0.4, 0.4),
                custom_size: Some(Vec2 { x: 1., y: 1. }),
                ..default()
            },
        tf: Transform::from_xyz(12.,12., 2.),
        data: WorkerData { coins: 0, target_coin_entity: Option::None, targer_coin_pos: Option::None, target_coin_dir: Option::None, house_pos: (0,0) }
    });

    cmm.spawn(WorkerBundle {
        spr: Sprite {
                color: Color::srgb(1., 0.4, 0.4),
                custom_size: Some(Vec2 { x: 1., y: 1. }),
                ..default()
            },
        tf: Transform::from_xyz(4.,12., 2.),
        data: WorkerData { coins: 0, target_coin_entity: Option::None, targer_coin_pos: Option::None, target_coin_dir: Option::None, house_pos: (0,0) }
    });

    cmm.spawn(WorkerBundle {
        spr: Sprite {
                color: Color::srgb(1., 0.4, 0.4),
                custom_size: Some(Vec2 { x: 1., y: 1. }),
                ..default()
            },
        tf: Transform::from_xyz(12.,4., 2.),
        data: WorkerData { coins: 0, target_coin_entity: Option::None, targer_coin_pos: Option::None, target_coin_dir: Option::None, house_pos: (0,0) }
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

    for (tf, mut data, ent) in &mut workers {
        if let Some(coin) = coins.iter().find(|c| (c.0.translation.x - tf.translation.x).norm() < distance && (c.0.translation.y - tf.translation.y).norm() < distance) {
            let dir = (tf.translation - coin.0.translation).normalize_or_zero();
            data.target_coin_dir = Some(dir);
        }
    }
}

fn wwwy(time: Res<Time>, mut workers: Query<(&mut Transform, &mut WorkerData), (With<WorkerData>, Without<WorkerCollectable>)>) {
    for (mut worker_tf,worker_data) in &mut workers {
        if worker_data.target_coin_dir != Option::None { worker_tf.translation -= worker_data.target_coin_dir.unwrap() * 1.0 * time.delta_secs(); }
    }
}