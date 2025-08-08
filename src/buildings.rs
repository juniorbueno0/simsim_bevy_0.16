use std::collections::HashSet;
use bevy::prelude::*;

use crate::{gameui::{ItemSelected, UiItemSlotButton, UiSlot}, mouse::{MyWorldCoords, PointingAtUi}, player::{ItemType, PlayerInventory}, worker::WorkerCollectable};

// #[derive(Resource, PartialEq, Eq)]
// pub struct Buildings {
//    pub data: HashSet<((i32,i32), BuildingType)>
// }

#[derive(Resource, PartialEq, Eq)]
pub struct Buildings {
   pub data: HashSet<(i32,i32)>
}

#[derive(Debug, Component, PartialEq, Eq, Hash)]
pub enum BuildingType {
    None,
    House,
    Dirt
}

#[derive(Debug, Component)]
pub struct HouseData {
    pub building_type: BuildingType,
    pub assigned_workers: HashSet<Entity>,
    pub max_capacity: i32
}

#[derive(Debug, Bundle)]
struct HouseBuildingBundle {
    tf: Transform,
    sprite: Sprite,
    data: HouseData,
}

#[derive(Debug, Component)]
pub enum CropType {
    Potato
}

#[derive(Debug, Component)]
pub struct PreparedDirtData {
    pub item_type: ItemType,
    pub crop_type: CropType,
    pub growth_state: i32,
    pub growth_complete: bool,
    pub growth_state_timer: Timer,
    pub worker_assigned: Entity
}

#[derive(Bundle)]
struct DirtBundle {
    tf: Transform,
    spr: Sprite,
    data: PreparedDirtData
}

pub struct MyBuildingPlugin;

impl Plugin for MyBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Buildings{data:HashSet::new()});

        app.add_systems(Update, spawn_items);
        app.add_systems(Update, log_buildings);
    }
}

fn spawn_items(
    cmm: Commands,
    item: Res<ItemSelected>,
    pointing_at: Res<PointingAtUi>,
    mut inv: ResMut<PlayerInventory>,
    mut buildings: ResMut<Buildings>,
    world_coords: Res<MyWorldCoords>,
    input: Res<ButtonInput<MouseButton>>,
    mut ui_buttons: Query<(&mut UiSlot, Entity), With<UiItemSlotButton>>
) {
    if input.just_pressed(MouseButton::Left) && item.selected != ItemType::None && pointing_at.can_place && !buildings.data.contains(&((world_coords.0.x as i32, world_coords.0.y as i32))) {

        let mut spawn_entity = |item_type: ItemType, mut cmm: Commands| {
            let building = match item_type {
                ItemType::Coin => {
                    cmm.spawn((
                        Sprite {
                            color: Color::srgb(1., 0.98, 0.),
                            custom_size: Some(Vec2 { x: 1., y: 1. }),
                            ..default()
                        },
                        Transform::from_xyz(world_coords.0.x, world_coords.0.y, 2.),
                        WorkerCollectable,
                        ItemType::Coin
                    ));
                    true
                },
                ItemType::House => {
                    cmm.spawn(HouseBuildingBundle {
                        sprite: Sprite { color:Color::srgb(0.9, 0.9, 0.8), custom_size: Some(vec2(1., 1.)), ..default() },
                        tf: Transform::from_xyz(world_coords.0.x, world_coords.0.y, 2.),
                        data: HouseData {
                            building_type: BuildingType::House,
                            assigned_workers: HashSet::new(),
                            max_capacity: 2
                        }
                    });
                    true
                },
                ItemType::Dirt=> {
                    cmm.spawn(DirtBundle {
                        spr: Sprite { color:Color::srgb(0.7, 0.5, 0.0), custom_size: Some(vec2(1., 1.)), ..default() },
                        tf: Transform::from_xyz(world_coords.0.x, world_coords.0.y, 2.),
                        data: PreparedDirtData {
                            item_type: ItemType::Dirt,
                            crop_type: CropType::Potato, // add none as default later
                            growth_state: 0,
                            growth_state_timer: Timer::from_seconds(60., TimerMode::Once),
                            growth_complete: false,
                            worker_assigned: Entity::from_raw(0)
                        }
                    });
                    true
                },
                _=> { false }
            };

            if building { buildings.data.insert((world_coords.0.x as i32, world_coords.0.y as i32)); }
        };

        if let Some(stack) = inv.items.iter_mut().find(|i| (i.item != ItemType::None) && (i.total_amount >= 1) && (i.ui_entity == item.entity)) {
            stack.total_amount -= 1;
            if let Some(mut ui_slot) = ui_buttons.iter_mut().find(|(slot,e)| (*e == stack.ui_entity) && slot.amount >= 1) {
                ui_slot.0.amount -= 1;
                spawn_entity(item.selected, cmm);
            }
        }
    }
}

fn log_buildings(input: Res<ButtonInput<KeyCode>>, buildings: Res<Buildings>) {
    if input.just_pressed(KeyCode::KeyP) {
        for b in &buildings.data {
            println!("building: {:?}", b);
        }
    }
}