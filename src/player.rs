use bevy::{platform::collections::HashSet, prelude::*};

pub const INVENTORYSIZE: i32 = 6;

#[derive(Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum Item {
    None,
    Coin,
    House
}

#[derive(Debug, Component)]
pub struct ItemStack {
    pub item: Item,
    pub total_amount: i32, 
    pub max_amount: i32, // max possible amount in 1 slot
    pub assigned: bool // ui uses this to bind it to the ui
}

#[derive(Debug, Resource)]
pub struct PlayerInventory {
    pub items: Vec<ItemStack>,
    pub size: i32
}

pub struct MyPlayerPlugin;

impl Plugin for MyPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInventory { items: vec![], size: INVENTORYSIZE });
        
        app.add_systems(Startup, init);
        app.add_systems(Update, inventory_log);
    }
}

fn init(mut inventory: ResMut<PlayerInventory>) {
    inventory.items.push(ItemStack { item: Item::Coin, total_amount: 4, max_amount: 9, assigned: false });
}

fn inventory_log(input: Res<ButtonInput<KeyCode>>, inv: Res<PlayerInventory>) {
    if input.just_pressed(KeyCode::KeyP) {
        println!("[player_inventory] {:?}", inv);
    }
}