use std::option;

use bevy::{math::NormedVectorSpace, platform::collections::HashSet, prelude::*};

use crate::{buildings::{BuildingType, Buildings, HouseData, PreparedDirtData}, player::{CoinsSpawned, ItemType}, world::{Meridiem, WorldSettings}};

#[derive(Debug, Resource)]
pub struct WorkerAmount { total: i32 }

#[derive(Debug, Component)]
pub struct WorkerCollectable;

#[derive(Component)]
struct Employed;

#[derive(Component)]
struct Working;

#[derive(Component)]
pub struct WorkerData {
    pub coins: i32,
    pub house_pos: (i32,i32),
    pub house_assigned: bool,
    pub target_crop_pos: Option<Vec3>,
    pub target_crop_entity: Option<Entity>,
    pub target_crop_active: bool, // is closer to the crop?
    pub target_coin_pos: Option<Vec3>,
    pub target_coin_dir: Option<Vec3>,
    pub target_coin_entity: Option<Entity>,
}

#[derive(Bundle)]
pub struct WorkerBundle {
    pub spr: Sprite,
    pub tf: Transform,
    pub data: WorkerData
}

pub struct MyWorkerPlugin;

impl Plugin for MyWorkerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorkerAmount { total: 0 });
        app.add_systems(Startup, setup);
        app.add_systems(Update, worker_amount_update);
        // SYSTEMS []
        app.add_systems(Update, (worker_collect_coin, worker_assign_home, worker_life_cycle));
        // app.add_systems(Update, (wwww,wwwx,wwwy,wwwz));
        // app.add_systems(Update, (wwwy,wwwz,worker_assign_home));
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
        data: WorkerData { 
            coins: 0,
            target_crop_pos: Option::None,
            target_crop_entity: Option::None,
            target_crop_active: false,
            target_coin_entity: Option::None, 
            target_coin_pos: Option::None, 
            target_coin_dir: Option::None, 
            house_pos: (0,0), 
            house_assigned: false 
        }
    });
}

fn worker_amount_update(
    buildings: Query<&BuildingType>,
    mut workers: ResMut<WorkerAmount>
) {
    let house_capacity = 1;
    let houses = buildings.iter().count();
    workers.total = (houses * house_capacity) as i32; // change later when houses can be upgraded
}

fn worker_collect_coin(
    time: Res<Time>, 
    mut cmm: Commands,
    mut coins_spawned: ResMut<CoinsSpawned>, // replace
    coins_query: Query<(&Transform, Entity, &ItemType), (With<WorkerCollectable>, Without<WorkerData>)>,
    mut workers: Query<(&mut Transform, &mut WorkerData, Entity), (With<WorkerData>, (Without<WorkerCollectable>, Without<Employed>))>, 
) {
    let worker_view_distance = 8.;

    let mut coins_assigned: HashSet<Entity> = HashSet::new(); // coins assigned to a worker
    let workers_iterator: Vec<_> = workers.iter_mut().collect(); 

    for (mut worker_tf, mut worker_data, worker_entity) in workers_iterator {

        if let Some(coin_data) = coins_query.iter().find(|c| 
            (c.0.translation.x - worker_tf.translation.x).norm() < worker_view_distance &&
            (c.0.translation.y - worker_tf.translation.y).norm() < worker_view_distance && 
            worker_data.coins < 1 && !coins_assigned.contains(&c.1)
        ) {
            let dir = (worker_tf.translation - coin_data.0.translation).normalize_or_zero();
            coins_assigned.insert(coin_data.1);
            worker_data.target_coin_dir = Some(dir);
            worker_data.target_coin_entity = Some(coin_data.1);
            worker_data.target_coin_pos = Some(coin_data.0.translation);
        };

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
            cmm.entity(worker_data.target_coin_entity.unwrap()).despawn();
            coins_spawned.positions.remove(&(worker_tf.translation.x as i32, worker_tf.translation.y as i32)); // not working properly
            cmm.entity(worker_entity).insert(Employed);

            coins_assigned.remove(&worker_data.target_coin_entity.unwrap());

            reset_worker_coin_data(&mut worker_data);
        }else { worker_tf.translation -= worker_dir * speed * time.delta_secs(); }
        
        if let Some(_) = coins_query.iter().find(|c|
            worker_data.target_coin_entity != Option::None && c.1==worker_data.target_coin_entity.unwrap()
        ) { }else { // if coin despawn then worker stops moving
            reset_worker_coin_data(&mut worker_data);
        }
    }
}

fn worker_assign_home(
    mut cmm: Commands,
    mut houses: Query<(&mut HouseData, &Transform, Entity), (With<HouseData>, Without<Employed>)>,
    mut workers_query: Query<(&mut Transform, &mut WorkerData, Entity), With<Employed>>
) {
    if let Some(mut available_house) = houses.iter_mut().find(|house| house.0.assigned_workers.len() < 1) {

        let employes: Vec<_> = workers_query.iter_mut().collect();

        for mut employed_worker in employes {
            if employed_worker.1.coins == 1 && !employed_worker.1.house_assigned && available_house.0.assigned_workers.len() < 1 {
                available_house.0.assigned_workers.insert(employed_worker.2);
                employed_worker.1.house_pos = (available_house.1.translation.x as i32, available_house.1.translation.y as i32);
                employed_worker.0.translation = Vec3::new(available_house.1.translation.x, available_house.1.translation.y, 2.);
                employed_worker.1.house_assigned = true;
                cmm.entity(employed_worker.2).insert(Working);
                cmm.entity(employed_worker.2).remove::<Employed>();
            } 
        }
    }
}

fn worker_life_cycle(
    time: Res<Time>,
    day: Res<WorldSettings>,
    mut crops: Query<(&Transform, &mut PreparedDirtData, Entity), (With<PreparedDirtData>, Without<Working>)>,
    mut worker: Query<(&mut Transform, &mut WorkerData, Entity), (With<Working>, Without<Employed>)>
) {
    for mut w in worker.iter_mut() {
        
        // assign
        if let Some(mut crop) = crops.iter_mut().find(|c|!c.1.worker_assigned_bool && w.1.target_crop_pos == Option::None) {
            crop.1.worker_assigned_bool = true;
            crop.1.worker_assigned_entity = w.2;
            w.1.target_crop_pos = Some(crop.0.translation);
            w.1.target_crop_entity = Some(crop.2);
        };

        //move
        if w.1.target_crop_pos != Option::None {
            let is_work_time =
                (day.meridiem == Meridiem::AM && day.actual_hour >= 8.0) ||
                (day.meridiem == Meridiem::PM && day.actual_hour < 9.0);

            if is_work_time {
                let dir = w.0.translation - w.1.target_crop_pos.unwrap();
                w.0.translation -= dir * time.delta_secs();
                if dir.x < 1.0 && dir.y < 1. { w.1.target_crop_active = true; }else { w.1.target_crop_active = false; } // crop growth active
            } else {
                let dir = w.0.translation - Vec3::new(w.1.house_pos.0 as f32,w.1.house_pos.1 as f32,2.);
                w.0.translation -= dir * time.delta_secs();
            }
        }
    }
}

// // all this part neds to be rewrited
// fn wwwz(mut workers: Query<(&mut Transform, &mut WorkerData,), (With<WorkerData>, Without<WorkerCollectable>)>, coins: Query<(&Transform, Entity, &ItemType), (With<WorkerCollectable>, Without<WorkerData>)>) {
//     let distance: f32 = 8.;

//     for (tf, mut data) in &mut workers {

//         if let Some(coin) = coins.iter().find(|c| (c.0.translation.x - tf.translation.x).norm() < distance && (c.0.translation.y - tf.translation.y).norm() < distance && data.coins < 1) {
//             let dir = (tf.translation - coin.0.translation).normalize_or_zero();
//             data.target_coin_dir = Some(dir);
//             data.target_coin_pos = Some(coin.0.translation);
//             data.target_coin_entity = Some(coin.1);
//         }
//     }
// }

// fn wwwy(time: Res<Time>, mut cmm: Commands, mut coins_spawned: ResMut<CoinsSpawned>, mut workers: Query<(&mut Transform, &mut WorkerData, Entity), (With<WorkerData>, Without<WorkerCollectable>)>, coins: Query<Entity, (With<WorkerCollectable>, Without<WorkerData>)>) {
//     for (mut worker_tf,mut worker_data, worker_entity) in &mut workers {
//         if worker_data.target_coin_dir == Option::None || worker_data.target_coin_pos == Option::None || worker_data.target_coin_entity == Option::None { continue; }

//         let speed = 1.0;
//         let offset = 0.1;
//         let worker_pos = worker_data.target_coin_pos.unwrap();
//         let worker_dir = worker_data.target_coin_dir.unwrap();

//         let reset_worker_coin_data = | worker: &mut WorkerData | {
//             worker.target_coin_dir = Option::None;
//             worker.target_coin_pos = Option::None;
//             worker.target_coin_entity = Option::None;
//         };

//         if worker_tf.translation.x > (worker_pos.x - offset) && worker_tf.translation.x < (worker_pos.x + offset) {
//             worker_data.coins += 1;
//             println!(" collected a coin ");
//             cmm.entity(worker_data.target_coin_entity.unwrap()).despawn();
//             coins_spawned.positions.remove(&(worker_tf.translation.x as i32, worker_tf.translation.y as i32)); // not working properly
//             cmm.entity(worker_entity).insert(Employed);
//             reset_worker_coin_data(&mut worker_data);
//         }else { worker_tf.translation -= worker_dir * speed * time.delta_secs(); }
        
//         if let Some(_) = coins.iter().find(|c|worker_data.target_coin_entity != Option::None && *c == worker_data.target_coin_entity.unwrap()) { }else { // if coin despawn then worker stops moving
//             reset_worker_coin_data(&mut worker_data);
//         }
//     }
// }

// // assign it to an available home
// fn wwwx(mut houses: Query<(&mut HouseData, &Transform)>, mut workers: Query<(&Transform, &mut WorkerData, Entity), (With<Employed>, Without<WorkerCollectable>)>) {
//     if let Some(mut house) = houses.iter_mut().find(|(h, _)| h.building_type == BuildingType::House && h.assigned_workers.len() < h.max_capacity as usize ) {
//         if let Some(mut worker) = workers.iter_mut().find(|w|w.1.coins == 1 && w.1.house_pos == (0,0)) {
//             worker.1.house_pos = (house.1.translation.x as i32,house.1.translation.y as i32);
//             house.0.assigned_workers.insert(worker.2);
//             println!("assigned: {:?}", worker.2);
//         }
//     } else { return; }
// }

// // the worker walks to his house
// fn wwww(time: Res<Time>, mut workers: Query<(&mut Transform, &WorkerData, Entity), (With<WorkerData>, Without<WorkerCollectable>)>) {

//     for mut w in &mut workers { // change it to filter thern iter
//         if w.1.coins == 1 && w.1.house_pos != (0,0) {
//             let dir = w.0.translation - Vec3::new(w.1.house_pos.0 as f32,w.1.house_pos.1 as f32, 1.);
//             w.0.translation -= dir * time.delta_secs();
//         }
//     }
// }

// // working cicle
