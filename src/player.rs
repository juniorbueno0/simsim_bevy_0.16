use bevy::prelude::*;

#[derive(Debug, Component)]
pub enum Item {
    Coin,
    House
}

#[derive(Debug, Component)]
struct ItemStack {
    item: Item,
    total_amount: i32,
    max_amount: i32
}

#[derive(Debug, Resource)]
pub struct PlayerInventory {
    items: Vec<ItemStack>,
    size: i32
}

pub struct MyPlayerPlugin;

impl Plugin for MyPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerInventory { items: vec![], size: 6 });
        
        app.add_systems(Startup, init);
        app.add_systems(Update, inventory_log);
    }
}

fn init(mut inventory: ResMut<PlayerInventory>) {
    inventory.items.push(ItemStack { item: Item::House, total_amount: 1, max_amount: 4 });
}

fn inventory_log(input: Res<ButtonInput<KeyCode>>, inv: Res<PlayerInventory>){
    if input.just_pressed(KeyCode::KeyP) {
        println!("[player_inventory] {:?}", inv);
    }
}