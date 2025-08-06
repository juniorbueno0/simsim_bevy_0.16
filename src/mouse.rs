use bevy::{prelude::*, window::{PrimaryWindow, WindowResized}};

use crate::camera::MainCamera;

#[derive(Resource, Debug)]
pub struct MyWorldCoords(pub Vec2);

// #[derive(Resource, Debug)]
// struct MouseWindowPosition(Vec2);

#[derive(Debug, Resource)]
struct WindowProperties {
    res: (f32, f32)
}

#[derive(Debug, Resource)]
pub struct PointingAtUi {
    pub can_place: bool
}

#[derive(Component)]
#[require(Sprite, Transform)]
struct MousePixelPosition;

pub struct MyMousePlugin;

impl Plugin for MyMousePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PointingAtUi { can_place: false });
        app.insert_resource(MyWorldCoords(Vec2 { x:0., y:0. }));
        app.insert_resource(WindowProperties { res: (0.0, 0.0) });
        // app.insert_resource(MouseWindowPosition(Vec2::new(0.,0.)));

        app.add_systems(Startup, setup);
        app.add_systems(Update,(cursor_to_world_position, mouse_pixel_position, can_build_here));
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
            // image: ass.load("ph.png"), 
            custom_size: Some(Vec2::new(1.,1.)), 
            ..default() 
        }
    ));
}

// convert the mouse coords to window coords
fn cursor_to_world_position(
    mut mycoords: ResMut<MyWorldCoords>,
    mut cursor_events: EventReader<CursorMoved>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let Some(_) = cursor_events.read().last() else { return };

    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single().unwrap();
    let window = q_window.single().unwrap();

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
        mycoords.0 = Vec2::new(x_value as f32, y_value as f32);
    }
}

fn can_build_here(
    mut pointing_at: ResMut<PointingAtUi>,
    resize_event: Res<Events<WindowResized>>,
    mut window_res: ResMut<WindowProperties>,
    mut cursor_event: EventReader<CursorMoved>,
) {
    let mut reader = resize_event.get_cursor();
    
    for re in reader.read(&resize_event) {
        window_res.res = (re.width, re.height);
    }

    if let Some(cursor) = cursor_event.read().next() {
        // println!("cursor: {:?}",cursor);
        // println!("resolution: {:?}", window_res.res);

        let mouse = cursor.position;
        let perc_y = window_res.res.1 / 100.;
        let perc_x = window_res.res.0 / 100.;

        if ((mouse.x < (perc_x * 4.)) || (mouse.y < (perc_y * 26.))) || (mouse.y > (perc_y * 78.) || (mouse.x > (perc_x * 11.))) {
            pointing_at.can_place = true;
            return;
        }

        pointing_at.can_place = false;
    };
}

fn mouse_pixel_position(
    pixel: Res<MyWorldCoords>,
    mut pixel_transform: Query<&mut Transform, With<MousePixelPosition>>
) {
    if let Ok(mut tf) = pixel_transform.single_mut() {
        tf.translation = Vec3::new(pixel.0.x,pixel.0.y, 4.);
    };
}