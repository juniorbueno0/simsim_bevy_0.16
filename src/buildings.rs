use std::collections::HashSet;
use bevy::prelude::*;

use crate::{gameui::{ItemSelected, UiItemSlotButton, UiSlot}, mouse::{MyWorldCoords, PointingAtUi}, player::{Item, PlayerInventory}};

#[derive(Resource, PartialEq, Eq)]
pub struct Buildings {
   pub data: HashSet<((i32,i32), BuildingType)>
}

#[derive(Debug, Component, PartialEq, Eq, Hash)]
pub enum BuildingType {
    House
}

#[derive(Debug, Bundle)]
struct BuildingBundle {
    tf: Transform,
    sprite: Sprite,
    tipe: BuildingType,
}

pub struct MyBuildingPlugin;

impl Plugin for MyBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Buildings{data:HashSet::new()});

        app.add_systems(Update, (log_buildings, spawn_houses)); 
    }
}

fn spawn_houses(
    mut cmm: Commands,
    item: Res<ItemSelected>,
    pointing_at: Res<PointingAtUi>,
    mut inv: ResMut<PlayerInventory>,
    world_coords: Res<MyWorldCoords>,
    mut buildings: ResMut<Buildings>,
    input: Res<ButtonInput<MouseButton>>,
    mut ui_buttons: Query<(&mut UiSlot, Entity), With<UiItemSlotButton>>
) {
    if input.just_pressed(MouseButton::Left) && item.selected == Item::House && pointing_at.can_place && !buildings.data.contains(&((world_coords.0.x as i32, world_coords.0.y as i32), BuildingType::House)) {
        buildings.data.insert(((world_coords.0.x as i32, world_coords.0.y as i32), BuildingType::House));

        if let Some(stack) = inv.items.iter_mut().find(|i| (i.item == Item::House) && (i.total_amount >= 1) && (i.ui_entity == item.entity)) {
            stack.total_amount -= 1;
            if let Some(mut ui_slot) = ui_buttons.iter_mut().find(|(slot,e)| (*e == stack.ui_entity) && slot.amount >= 1) {
                ui_slot.0.amount -= 1;
                cmm.spawn(BuildingBundle {
                    sprite: Sprite { color:Color::srgb(0.9, 0.9, 0.8), custom_size: Some(vec2(1., 1.)), ..default() },
                    tf: Transform::from_xyz(world_coords.0.x, world_coords.0.y, 2.),
                    tipe: BuildingType::House
                });

                buildings.data.insert(((world_coords.0.x as i32, world_coords.0.y as i32), BuildingType::House));
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