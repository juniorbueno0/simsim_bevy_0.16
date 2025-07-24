use std::collections::HashSet;
use bevy::prelude::*;

#[derive(Resource, PartialEq, Eq)]
struct Buildings {
   data: HashSet<((i32,i32), BuildingType)>
}

#[derive(Debug, Component, PartialEq, Eq, Hash)]
enum BuildingType {
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
        app.add_systems(Startup, init);
        app.add_systems(Update, log_buildings); 
    }
}

fn init(mut cmm: Commands, mut buildings: ResMut<Buildings>) {
    cmm.spawn(
        BuildingBundle {
            sprite: Sprite { color:Color::srgb(0.9, 0.9, 0.8), custom_size: Some(vec2(1., 1.)), ..default() },
            tf: Transform::from_xyz(0., 0., 1.),
            tipe: BuildingType::House
        }
    );
    buildings.data.insert(((0,0), BuildingType::House));
}

fn log_buildings(input: Res<ButtonInput<KeyCode>>, buildings: Res<Buildings>) {
    if input.just_pressed(KeyCode::KeyP) {
        for b in &buildings.data {
            println!("building: {:?}", b);
        }
    }
}