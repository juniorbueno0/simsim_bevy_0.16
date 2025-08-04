use bevy::prelude::*;

use crate::player::{Item, PlayerInventory, INVENTORYSIZE};

#[derive(Component)]
struct UiButton;

#[derive(Component)]
pub struct UiItemSlotButton;

#[derive(Resource, Debug)]
pub struct ItemSelected {
    pub selected: Item,
    pub entity: Entity
}

#[derive(Debug, Component)]
pub struct UiSlot {
    pub item: Item,
    pub amount: i32,
    pub assigned: bool
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
        app.insert_resource(ItemSelected { selected: Item::None, entity: Entity::from_raw(0) });
 
        app.add_systems(Startup, ui_setup);      
        app.add_systems(Update, (ui_slot_interactions, ui_load_items, ui_reset_slot, reset_player_item_selected));
    }
}

fn ui_setup(mut commands: Commands) {
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

fn ui_slot_interactions(
    mut selection: ResMut<ItemSelected>,
    button_interactions: Query<(&Interaction,&UiSlot,Entity),(With<UiItemSlotButton>, Changed<Interaction>)>
) {
    for (int, uis, ent) in &button_interactions {
        if *int == Interaction::Pressed {
            (selection.selected, selection.entity) = (uis.item, ent);

            println!("{:?}, {:?}", uis, ent);
        }
    }
}

// load items that are in the inventory
fn ui_load_items(
    mut slots: Query<(&mut UiSlot, Entity), With<UiItemSlotButton>>,
    mut player_inventory: ResMut<PlayerInventory>
) {
    let Some(item_not_assigned) = player_inventory.items.iter_mut().find(|i|!i.assigned) else { return; };

    if let Some(mut slot) = slots.iter_mut().find(|s| s.0.item == Item::None) {
        slot.0.assigned = true;
        slot.0.item = item_not_assigned.item;
        slot.0.amount = item_not_assigned.total_amount;

        item_not_assigned.assigned = true; // not assign it again!
        item_not_assigned.ui_entity = slot.1; // assign the entity id of the ui slot to later decrease it
    }
}

// when inventory stack gets to 0 gets reset to Item::None 
fn ui_reset_slot(
    mut ui_slots: Query<&mut UiSlot, With<UiItemSlotButton>>,
) {
    if let Some(mut ui_button) = ui_slots.iter_mut().find(|uib|(uib.amount < 1) && (uib.item != Item::None)) {
        ui_button.item = Item::None;
        ui_button.assigned = false;
    };
}

fn reset_player_item_selected(
    mut player_selected_item: ResMut<ItemSelected>,
    ui_slots: Query<(&UiSlot,Entity),With<UiItemSlotButton>>
) {
    let entity = ui_slots.iter().find(|s|(s.1 == player_selected_item.entity) && (s.0.item != Item::None));

    match entity {
        Option::Some(_) => {},
        Option::None => { player_selected_item.selected = Item::None; player_selected_item.entity = Entity::from_raw(0) ; }
    }
}