use bevy::{platform::collections::HashSet, prelude::*};

use crate::gameui::{ItemSelected, UiItemSlotButton, UiSlot};
use crate::mouse::{MyWorldCoords, PointingAtUi};
use crate::worker::WorkerCollectable;

pub const INVENTORYSIZE: i32 = 6;
const MAXSTACKSIZE: i32 = 999;

#[derive(Debug, Component)]
pub enum Tool {
    Shovel
}

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum ItemType {
    None,
    Coin,
    House,
    Shovel,
    Dirt,
    Worker // testing
}

#[derive(Debug, Resource)]
pub struct CoinsSpawned {
    pub positions: HashSet<(i32,i32)>
}

#[derive(Debug, Component)]
pub struct ItemStack {
    pub item: ItemType,
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
        app.add_systems(Update, (inventory_log, despawn_inventory_stack));
    }
}

fn init(mut inventory: ResMut<PlayerInventory>) {
    inventory.items.push(ItemStack { item: ItemType::Coin, total_amount: 64, max_amount: MAXSTACKSIZE, assigned: false, ui_entity: Entity::from_raw(0) });
    inventory.items.push(ItemStack { item: ItemType::House, total_amount: 32, max_amount: MAXSTACKSIZE, assigned: false, ui_entity: Entity::from_raw(0) });
    inventory.items.push(ItemStack { item: ItemType::Dirt, total_amount: 16, max_amount: MAXSTACKSIZE, assigned: false, ui_entity: Entity::from_raw(0) });
    inventory.items.push(ItemStack { item: ItemType::Worker, total_amount: 999, max_amount: MAXSTACKSIZE, assigned: false, ui_entity: Entity::from_raw(0) });
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