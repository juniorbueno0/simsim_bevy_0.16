use bevy::{platform::collections::HashSet, prelude::*};

use crate::gameui::{ItemSelected, UiItemSlotButton, UiSlot};
use crate::mouse::{MyWorldCoords, PointingAtUi};
use crate::worker::WorkerCollectable;

pub const INVENTORYSIZE: i32 = 6;
const MAXSTACKSIZE: i32 = 999;

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum Item {
    None,
    Coin,
    House
}

#[derive(Debug, Resource)]
pub struct CoinsSpawned {
    pub positions: HashSet<(i32,i32)>
}

#[derive(Debug, Component)]
pub struct ItemStack {
    pub item: Item,
    pub total_amount: i32, 
    pub max_amount: i32, // max possible amount in 1 slot
    pub assigned: bool, // ui uses this to bind it to the ui
    pub ui_entity: Entity
}

#[derive(Debug, Resource)]
pub struct PlayerInventory {
    pub items: Vec<ItemStack>,
    pub size: i32
}

pub struct MyPlayerPlugin;

impl Plugin for MyPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CoinsSpawned { positions: HashSet::new() });
        app.insert_resource(PlayerInventory { items: vec![], size: INVENTORYSIZE });

        app.add_systems(Startup, init);
        app.add_systems(Update, (inventory_log, drop_coin, despawn_inventory_stack));
    }
}

fn init(mut inventory: ResMut<PlayerInventory>) {
    inventory.items.push(ItemStack { item: Item::Coin, total_amount: 999, max_amount: MAXSTACKSIZE, assigned: false, ui_entity: Entity::from_raw(0) });
    inventory.items.push(ItemStack { item: Item::Coin, total_amount: 999, max_amount: MAXSTACKSIZE, assigned: false, ui_entity: Entity::from_raw(0) });
    inventory.items.push(ItemStack { item: Item::House, total_amount: 444, max_amount: MAXSTACKSIZE, assigned: false, ui_entity: Entity::from_raw(0) });
}

fn drop_coin(
    mut cmm: Commands, 
    item: Res<ItemSelected>,
    pointing_at: Res<PointingAtUi>,
    mut coins: ResMut<CoinsSpawned>,
    mut inv: ResMut<PlayerInventory>,
    world_coords: Res<MyWorldCoords>,
    input: Res<ButtonInput<MouseButton>>,
    mut ui_buttons: Query<(&mut UiSlot, Entity), With<UiItemSlotButton>>
) {
    if input.just_pressed(MouseButton::Left) && item.selected == Item::Coin && pointing_at.can_place &&
        !coins.positions.contains(&(world_coords.0.x as i32, world_coords.0.y as i32)) {

        if let Some(stack) = inv.items.iter_mut().find(|i| (i.item == Item::Coin) && (i.total_amount >= 1) && (i.ui_entity == item.entity)) {
            stack.total_amount -= 1;
            if let Some(mut ui_slot) = ui_buttons.iter_mut().find(|(slot,e)| (*e == stack.ui_entity) && slot.amount >= 1) {
                ui_slot.0.amount -= 1;

                cmm.spawn((
                    Sprite {
                        color: Color::srgb(1., 0.98, 0.),
                        custom_size: Some(Vec2 { x: 1., y: 1. }),
                        ..default()
                    },
                    Transform::from_xyz(world_coords.0.x, world_coords.0.y, 2.),
                    WorkerCollectable,
                    Item::Coin
                ));
                coins.positions.insert((world_coords.0.x as i32, world_coords.0.y as i32));
            }
        }
    }
}

// if a player inventory gets to 0 it gets removed
fn despawn_inventory_stack(
    mut player_inv: ResMut<PlayerInventory>,
) {
    match player_inv.items.iter().position(|i|i.total_amount <= 0) {
        Option::Some(index) => { player_inv.items.remove(index); },
        Option::None => {}
    }
}

fn inventory_log(input: Res<ButtonInput<KeyCode>>, inv: Res<PlayerInventory>, sel: Res<ItemSelected>) {
    if input.just_pressed(KeyCode::KeyP) {
        println!("[player_inventory] {:?}", inv);
        println!("[item selected] {:?}", sel);
    }
}