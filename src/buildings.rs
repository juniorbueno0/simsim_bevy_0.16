use std::collections::HashSet;
use bevy::prelude::*;

use crate::{crop::{CropType, DirtBundle, PreparedDirtData}, gameui::{ItemSelected, UiItemSlotButton, UiSlot}, mouse::{MyWorldCoords, PointingAtUi}, player::{ItemType, PlayerInventory}, worker::{WorkerBundle, WorkerCollectable, WorkerData}};
#[derive(Debug, Component)]
pub struct HasDynamicMenu;

#[derive(Resource, PartialEq, Eq)]
pub struct BuildingCoords {
   pub data: HashSet<(i32,i32)>
}

#[derive(Resource, PartialEq, Eq)]
pub struct BuildingTuple {
   pub data: HashSet<((i32,i32), ItemType)>
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

pub struct MyBuildingPlugin;

impl Plugin for MyBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(BuildingCoords { data:HashSet::new() });
        app.insert_resource(BuildingTuple { data:HashSet::new() });

        app.add_systems(Update, spawn_items);
    }
}

fn spawn_items(
    cmm: Commands,
    item: Res<ItemSelected>,
    pointing_at: Res<PointingAtUi>,
    mut inv: ResMut<PlayerInventory>,
    mut building_coords: ResMut<BuildingCoords>,
    mut buildings_tuple: ResMut<BuildingTuple>,
    world_coords: Res<MyWorldCoords>,
    input: Res<ButtonInput<MouseButton>>,
    mut ui_buttons: Query<(&mut UiSlot, Entity), With<UiItemSlotButton>>
) {
    if input.just_pressed(MouseButton::Left) && item.selected != ItemType::None && pointing_at.can_place && !building_coords.data.contains(&((world_coords.0.x as i32, world_coords.0.y as i32))) {

        let mut spawn_entity = |item_type: ItemType, mut cmm: Commands| {
            let spawned = match item_type {
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
                    (true, ItemType::Coin)
                },
                ItemType::House => {
                    cmm.spawn(HouseBuildingBundle {
                        sprite: Sprite { color:Color::srgb(0.9, 0.9, 0.8), custom_size: Some(vec2(1., 1.)), ..default() },
                        tf: Transform::from_xyz(world_coords.0.x, world_coords.0.y, 1.),
                        data: HouseData {
                            building_type: BuildingType::House,
                            assigned_workers: HashSet::new(),
                            max_capacity: 2
                        }
                    });
                    (true, ItemType::House)
                },
                ItemType::Dirt=> {
                    cmm.spawn((DirtBundle {
                        spr: Sprite { color:Color::srgb(0.7, 0.5, 0.0), custom_size: Some(vec2(1., 1.)), ..default() },
                        tf: Transform::from_xyz(world_coords.0.x, world_coords.0.y, 1.),
                        data: PreparedDirtData {
                            item_type: ItemType::Dirt,
                            crop_type: CropType::Potato, // add none as default later
                            crop_type_selected: false, // player alredy selected one type to grow
                            growth_state: 0,
                            growth_active: false,
                            growth_state_timer: Timer::from_seconds(60., TimerMode::Once),
                            growth_complete: false,
                            worker_assigned_bool: false,
                            worker_assigned_entity: Entity::from_raw(0)
                        }
                    }, HasDynamicMenu));
                    (true, ItemType::Dirt)
                },
                ItemType::Worker => {
                    cmm.spawn(WorkerBundle {
                        spr: Sprite {
                                color: Color::srgb(1., 0.4, 0.4),
                                custom_size: Some(Vec2 { x: 0.5, y: 0.5 }),
                                ..default()
                            },
                        tf: Transform::from_xyz(world_coords.0.x, world_coords.0.y, 2.),
                        data: WorkerData { 
                            coins: 0, 
                            worker_speed: 1.0,
                            target_coin_entity: Option::None, 
                            target_coin_pos: Option::None,
                            target_coin_dir: Option::None,
                            house_pos: (0,0),
                            house_assigned: false,
                            target_crop_entity: Option::None,
                            target_crop_active: false,
                            target_crop_pos: Option::None
                        }
                    });
                    (false, ItemType::None)
                }
                _=> { (false, ItemType::None) }
            };

            if spawned.0 { 
                building_coords.data.insert((world_coords.0.x as i32, world_coords.0.y as i32));
                buildings_tuple.data.insert(((world_coords.0.x as i32, world_coords.0.y as i32), spawned.1));
            }
        };

        if let Some(stack) = inv.items.iter_mut().find(|i| (i.item != ItemType::None) && (i.total_amount >= 1) && (i.ui_entity == item.ui_entity)) {
            stack.total_amount -= 1;
            if let Some(mut ui_slot) = ui_buttons.iter_mut().find(|(slot,e)| (*e == stack.ui_entity) && slot.amount >= 1) {
                ui_slot.0.amount -= 1;
                spawn_entity(item.selected, cmm);
            }
        }
    }
}
