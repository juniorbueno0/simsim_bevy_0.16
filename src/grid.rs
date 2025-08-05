use std::collections::HashSet;
use noise::{NoiseFn, Perlin};

use bevy::prelude::*;

use crate::camera::MainCameraActualPosition;

#[derive(Debug, Resource)]
struct PerlinInstance { value: Perlin }

#[derive(Resource)]
struct LoadedChunks(HashSet<(i32, i32)>);

#[derive(Resource)]
struct DesiredChunks(HashSet<(i32,i32)>);

#[derive(Component)]
pub struct ChunkMarker { pub chunk_coords: (i32, i32) }

pub struct MyGridPlugin;

impl Plugin for MyGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedChunks(HashSet::new()));
        app.insert_resource(DesiredChunks(HashSet::new()));
        app.insert_resource(PerlinInstance{value:Perlin::new(4)});
        
        app.add_systems(Update, (generate_new_chunk_data, spawn_new_chunks, delete_old_chunks));
    }
}

const CHUNK_SIZE: i32 = 8;
const RENDER_DISTANCE: i32 = 4;

fn generate_new_chunk_data(mut desired_chunks:ResMut<DesiredChunks>,cam_main:Res<MainCameraActualPosition>) {
    let camera_chunk_x = (cam_main.0.x / CHUNK_SIZE as f32).floor() as i32;
    let camera_chunk_y = (cam_main.0.y / CHUNK_SIZE as f32).floor() as i32;
    
    desired_chunks.0 = HashSet::new();
    for chunk_xx in (camera_chunk_x - RENDER_DISTANCE)..=(camera_chunk_x + RENDER_DISTANCE) {
        for chunk_yy in (camera_chunk_y - RENDER_DISTANCE)..=(camera_chunk_y + RENDER_DISTANCE) {
            desired_chunks.0.insert((chunk_xx, chunk_yy));
        }
    }
}

fn spawn_new_chunks(mut commands:Commands,mut loaded_chunks:ResMut<LoadedChunks>,desired_chunks:Res<DesiredChunks>,perlin:Res<PerlinInstance>,asse:Res<AssetServer>) {
    for &chunk_coords in desired_chunks.0.iter() {
        if !loaded_chunks.0.contains(&chunk_coords) {
            let (chunk_x, chunk_y) = chunk_coords;
            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    let world_x = (chunk_x * CHUNK_SIZE + x) as f32;
                    let world_y = (chunk_y * CHUNK_SIZE + y) as f32; 

                    let noise_x = (chunk_x * CHUNK_SIZE + x) as f64 / 200.0;
                    let noise_y = (chunk_y * CHUNK_SIZE + y) as f64 / 200.0;

                    let noise_value = perlin.value.get([noise_x as f64, noise_y as f64]);

                    commands.spawn((
                        Transform {
                            translation: Vec3::new(world_x, world_y, 0.),
                            scale: Vec3::new(1., 1., 1.),
                            ..default()
                        },
                        Sprite {
                            color: assign_color(noise_value),
                            // image: assign_image(noise_value, &asse),//asse.load("gd02.png"),
                            custom_size: Some(Vec2::new(1.,1.)),
                            ..default()
                        },
                        ChunkMarker { chunk_coords },
                    ));
                }
            }
            loaded_chunks.0.insert(chunk_coords);
        }
    }
}

fn delete_old_chunks(mut commands:Commands,query: Query<(Entity, &ChunkMarker), With<ChunkMarker>>,desired_chunks:Res<DesiredChunks>,mut loaded_chunks:ResMut<LoadedChunks>) {
    for (entity, marker) in query.iter() {
        if !desired_chunks.0.contains(&marker.chunk_coords) {
            commands.entity(entity).despawn_recursive();
            loaded_chunks.0.remove(&marker.chunk_coords);
        }
    }
} 

fn assign_color(value: f64) -> Color {
    if (-1.2..=-0.8).contains(&value) {
        Color::srgb(0.0, 0.0, 0.5)
    } else if (-0.8..=-0.5).contains(&value) {
        Color::srgb(0.0, 0.2, 0.8)
    } else if (-0.5..=-0.1).contains(&value) {
        Color::srgb(0.3, 0.5, 1.0)
    } else if (-0.1..=0.0).contains(&value) {
        Color::srgb(1.0, 0.9, 0.6)
    } else if (0.0..=0.4).contains(&value) {
        Color::srgb(0.56, 0.83, 0.43)
    } else if (0.4..=0.8).contains(&value) {
        Color::srgb(0.4, 0.65, 0.28)
    } else if (0.8..=1.2).contains(&value) {
        Color::srgb(0.8, 0.8, 0.8)
    } else {
        Color::srgb(1.0, 1.0, 1.0)
    }
}