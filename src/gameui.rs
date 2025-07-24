use bevy::prelude::*;

pub struct MyGameUiPlugin;

impl Plugin for MyGameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, draw_base_ui);      
    }
}

fn draw_base_ui(mut commands: Commands) {
    commands.spawn(
        Node { width: Val::Percent(100.), height: Val::Percent(100.),  display: Display::Flex, flex_direction: FlexDirection::Column, ..default() }, 
    ).with_children(|children| {
        children.spawn(
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(5.), 
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..default()
            }
        ).with_children(|cc| {
            cc.spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Px(24.), 
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                }, BackgroundColor(Color::srgb(0.3,0.3,0.3))
            ));
        });

        children.spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(95.),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..default()
            }, BackgroundColor(Color::srgb(0.4,0.4,0.6))
        ));
    });
}