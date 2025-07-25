use bevy::prelude::*;

use crate::player::{Item, PlayerInventory, INVENTORYSIZE};

#[derive(Component)]
struct UiButton;

#[derive(Component)]
struct UiItemSlotButton;

#[derive(Resource)]
struct ItemSelected {
    selected: Item
}

#[derive(Debug, Component)]
struct UiSlot {
    item: Item,
    amount: i32,
    assigned: bool
}

#[derive(Bundle)]
struct CustomUiButton {
    node: Node,
    data: UiSlot,
    text: Text,
    button: Button,
    id: UiItemSlotButton
}

pub struct MyGameUiPlugin;

impl Plugin for MyGameUiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ItemSelected { selected: Item::None });
 
        app.add_systems(Startup, draw_base_ui);      
        app.add_systems(Update, (ui_btn_interactions, load_ui_items));
    }
}

fn draw_base_ui(mut commands: Commands) {
    let inventory_row_gap = 10.0;
    let rgb_topbar = (0.3,0.3,0.3);
    let rgb_inventory_bg = (0.5,0.6,0.5);
    let rgb_inv_slot = (0.4,0.5,0.4);

    commands.spawn(
        Node { width: Val::Percent(100.), height: Val::Percent(100.),  display: Display::Flex, flex_direction: FlexDirection::Column, ..default() }, 
    ).with_children(|children: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>| {

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
                }, BackgroundColor(Color::srgb(rgb_topbar.0,rgb_topbar.1,rgb_topbar.2))
            ));
        });

        children.spawn(
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(95.),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..default()
            }
        ).with_children(|cc| {
            cc.spawn(
                Node {
                    width: Val::Percent(15.),
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                }
            )
            .with_children(|c| {
                c.spawn(
                (Node {
                        width: Val::Px(60.),
                        height: Val::Px(300.),
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        flex_wrap: FlexWrap::Wrap,
                        row_gap: Val::Px(inventory_row_gap),
                        ..default()
                    }, BackgroundColor(Color::srgb(rgb_inventory_bg.0,rgb_inventory_bg.1,rgb_inventory_bg.2)))
                ).with_children(|c| {
                    for _ in 0..INVENTORYSIZE {
                        c.spawn(build_custom_button(Item::None,0,rgb_inv_slot));
                    }
                });
            });

            cc.spawn(
                Node {
                    width: Val::Percent(85.),
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                }
            );
        });

    });
}

fn build_custom_button(item: Item, amount: i32,rgb: (f32,f32,f32)) -> impl Bundle  {
    (
        CustomUiButton {
            node: Node {
                width: Val::Px(40.),
                height: Val::Px(40.),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            data: UiSlot { item, amount, assigned: false },
            text: Text("".to_string()),
            button: Button,
            id: UiItemSlotButton
        }, 
        BackgroundColor(Color::srgb(rgb.0,rgb.1,rgb.2))
    )
}

fn ui_btn_interactions(
    mut selection: ResMut<ItemSelected>,
    button_interactions: Query<(&Interaction,&UiSlot,Entity),(With<UiItemSlotButton>, Changed<Interaction>)>
) {
    for (int, uis, ent) in &button_interactions {
        if *int == Interaction::Pressed {
            selection.selected = uis.item;

            println!("{:?}, {:?}", uis, ent);
        }
    }
}

// load items that are in the inventory
fn load_ui_items(
    mut slots: Query<&mut UiSlot, With<UiItemSlotButton>>,
    mut player_inventory: ResMut<PlayerInventory>
) {
    let Some(item_not_assigned) = player_inventory.items.iter_mut().find(|i|!i.assigned) else { return; };

    if let Some(mut slot) = slots.iter_mut().find(|s| s.item == Item::None) {
        slot.assigned = true;
        slot.item = item_not_assigned.item;
        slot.amount = item_not_assigned.total_amount;

        item_not_assigned.assigned = true; // not assign it again!
    }
}