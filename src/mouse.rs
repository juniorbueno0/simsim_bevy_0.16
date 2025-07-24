use bevy::{prelude::*, window::{PrimaryWindow, WindowResized}};

#[derive(Resource, Debug)]
pub struct MyWorldCoords(pub Vec2);

#[derive(Resource, Debug)]
struct MouseWindowPosition(Vec2);

// COMPONENTS 
#[derive(Component)]
#[require(Sprite, Transform)]
struct MousePixelPosition;

pub struct MyMousePlugin;

impl Plugin for MyMousePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MyWorldCoords(Vec2 { x:0., y:0. }));
        app.insert_resource(MouseWindowPosition(Vec2::new(0.,0.)));

        app.add_systems(Startup, setup);
        app.add_systems(Update,cursor_to_world_position);
    }
}

fn setup(mut commands:Commands, ass:Res<AssetServer>) {
    // pixel where the player is selecting
    commands.spawn((
        MousePixelPosition,
        Transform {
            translation: Vec3 { x: 0., y: 0., z: 0. },
            rotation: Quat::from_xyzw(0., -0., -0., 0.),
            scale: Vec3 { x: 1., y: 1., z: 1. }
        },
        Sprite { 
            color: Color::srgba(0.8, 0.8,0.8, 0.3), 
            image: ass.load("ph.png"), 
            custom_size: Some(Vec2::new(1.,1.)), 
            ..default() 
        }
    ));
}

// convert the mouse coords to window coords
fn cursor_to_world_position(
    mut mycoords: ResMut<MyWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
        .map(|ray| ray.unwrap().origin.truncate())
    {
        mycoords.0 = world_position + Vec2::new(0.5, 0.5);
        // to int 
        let x_value: i32 = mycoords.0.x as i32;
        let y_value: i32 = mycoords.0.y as i32;
        // to f32 
        // needs fix
        mycoords.0 = Vec2::new(x_value as f32, y_value as f32);
    }
}