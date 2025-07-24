use bevy::{image::ImageSamplerDescriptor, prelude::*, render::{settings::{Backends, RenderCreation, WgpuSettings}, RenderPlugin}, window::WindowResolution};

use crate::mouse::MyMousePlugin;

mod grid;
mod mouse;
mod camera;
mod player;
mod gameui;
mod buildings;

// not yet implemented
// enum GameState {
//     Game,
//     Pause,
//     MenuMyPlayerPlugin
// }

fn main() {
    let mut app: App = App::new();

    app.add_plugins(DefaultPlugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings{backends:Some(Backends::VULKAN),..default()}),
            ..default()
        }
    ).set(
        ImagePlugin { default_sampler: ImageSamplerDescriptor::nearest() }
    ).set(WindowPlugin { 
            primary_window: Some(Window {
                resolution:WindowResolution::new(800.,600.).with_scale_factor_override(1.),
                ..default()
            }),
            ..default()
        }
    ));

    app.add_plugins(grid::MyGridPlugin);
    app.add_plugins(mouse::MyMousePlugin);
    app.add_plugins(player::MyPlayerPlugin);
    app.add_plugins(camera::MyCameraPlugin);
    app.add_plugins(gameui::MyGameUiPlugin);
    app.add_plugins(buildings::MyBuildingPlugin);
    
    app.run();
}

// npc
// search for a coin
// if have a coin they go to a house
// spawn and patrol specific biome

// player
// can drop a coin
// can place houses

// house
// gives the town more npc spaces