use bevy::prelude::*;

use crate::{buildings::{BuildingCoords, BuildingTuple, HasDynamicMenu}, crop::PreparedDirtData, mouse::MyWorldCoords, player::{ItemType, PlayerInventory, INVENTORYSIZE}, world::WorldSettings};

const RGBINVSLOT: (f32,f32,f32) = (0.4,0.5,0.4);

#[derive(Debug, Component)]
enum DynamicButtonId {
    ButtonOne,
    ButtonTwo,
    ButtonThree,
    ButtonFour,
    ButtonFive,
    ButtonSix
}

enum DynamicOption {
    Potato,
    Close
}

struct DynamicMenuOptions {
    building: ItemType,
    action: &'static[DynamicOption]
}

const DYNAMICMENUOPTIONS: [DynamicMenuOptions; 1] = [
    DynamicMenuOptions { building: ItemType::Dirt, action: &[DynamicOption::Potato, DynamicOption::Close] }
];

// #[derive(Component)]
// struct UiButton;

#[derive(Component)]
pub struct UiItemSlotButton;

#[derive(Component)]
pub struct UiWorldTime;

#[derive(Resource, Debug)]
pub struct ItemSelected {
    pub selected: ItemType,
    pub ui_entity: Entity
}

#[derive(Resource, Debug)]
pub struct DynamicUi {
    selected: ItemType,
    world_entity: Entity,
    parent_ui_entity: Entity,
    actual_ui_entity: Entity,
    button_count: i32,
    open: bool
}

#[derive(Debug, Component)]
pub struct UiSlot {
    pub item: ItemType,
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
        app.insert_resource(ItemSelected { selected: ItemType::None, ui_entity: Entity::from_raw(0) });
        app.insert_resource(DynamicUi { selected: ItemType::None, world_entity: Entity::from_raw(0), parent_ui_entity: Entity::from_raw(0), actual_ui_entity: Entity::from_raw(0), button_count: 0, open: false });
        
        app.add_systems(Startup, ui_setup);     
        app.add_systems(Update, (ui_slot_interactions, ui_load_items, ui_reset_slot, reset_player_item_selected));
        app.add_systems(Update, (highlight_slot_selected, reset_selected_item));
        app.add_systems(Update, (ui_slot_text, ui_world_time_text));
        app.add_systems(Update, (dyn_ui_selection, display_dyn_ui_selected, dynamic_menu_actions));
    }
}

fn ui_setup(mut commands: Commands, mut dyn_ui: ResMut<DynamicUi>) {
    let inventory_row_gap = 10.0;
    let rgb_topbar = (0.3,0.3,0.3);
    let rgb_inventory_bg = (0.5,0.6,0.5);

    commands.spawn(
        Node { width: Val::Percent(100.), height: Val::Percent(100.),  display: Display::Flex, flex_direction: FlexDirection::Column, ..default() }, 
    ).with_children(|children: &mut bevy::ecs::relationship::RelatedSpawnerCommands<'_, ChildOf>| {
       // TOP
       children.spawn( 
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(5.),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..default()
            }
        ).with_children(|top| {
            top.spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Px(24.), 
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                }, BackgroundColor(Color::srgb(rgb_topbar.0,rgb_topbar.1,rgb_topbar.2))
            )).with_children(|cc|{
                // BUTTONS
                cc.spawn((
                    Node {
                        width: Val::Percent(40.),
                        height: Val::Px(24.), 
                        display: Display::Flex,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    }, BackgroundColor(Color::srgb(rgb_topbar.0,rgb_topbar.1,rgb_topbar.2))
                ));

                cc.spawn(
                    Node {
                        width: Val::Percent(60.),
                        height: Val::Px(24.), 
                        display: Display::Flex,
                        flex_direction: FlexDirection::RowReverse,
                        ..default()
                    }
                ).with_children(|ccc| {
                    // WORLD TIMER
                    ccc.spawn((
                        Node {
                            width: Val::Px(98.),
                            height: Val::Px(24.),
                            display: Display::Flex,
                            flex_direction: FlexDirection::RowReverse,
                            ..default()
                        }, BackgroundColor(Color::srgb(0.4,0.6,0.6)),
                        UiWorldTime,
                        Text2d::new("00:00")
                    ));
                });
            });
        });
        // MIDDLE
        children.spawn( 
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(85.),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..default()
            }
        ).with_children(|middle| {
            middle.spawn(
                Node {
                    width: Val::Percent(10.),
                    height: Val::Percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                }
            ).with_children(|inv| {
                inv.spawn((
                    Node {
                        width: Val::Px(60.),
                        height: Val::Px(300.),
                        display: Display::Flex,
                        row_gap: Val::Px(inventory_row_gap),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    }, 
                    BackgroundColor(Color::srgb(rgb_inventory_bg.0,rgb_inventory_bg.1,rgb_inventory_bg.2))
                )).with_children(|slots| {
                    for _ in 0..INVENTORYSIZE {
                    slots.spawn(build_custom_button(ItemType::None,0,RGBINVSLOT));
                }
                });
            });

            middle.spawn(Node {
                width: Val::Percent(80.),
                height: Val::Percent(100.),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..default()
            });

            middle.spawn(Node {
                width: Val::Percent(10.),
                height: Val::Percent(100.),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                ..default()
            });
        });
        // BOTTOM
        children.spawn(
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(10.),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            }
        ).with_children(|bottom| {
            // DYN UI PARENT
            dyn_ui.parent_ui_entity = bottom.spawn(
                Node {
                    width: Val::Px(300.),
                    height: Val::Px(60.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                }
            ).id();
        });
    });

}

fn build_custom_button(item: ItemType, amount: i32,rgb: (f32,f32,f32)) -> impl Bundle  {
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
            (selection.selected, selection.ui_entity) = (uis.item, ent);

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

    if let Some(mut slot) = slots.iter_mut().find(|s| s.0.item == ItemType::None) {
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
    if let Some(mut ui_button) = ui_slots.iter_mut().find(|uib|(uib.amount < 1) && (uib.item != ItemType::None)) {
        ui_button.item = ItemType::None;
        ui_button.assigned = false;
    };
}

fn ui_slot_text(
    mut ui_slots: Query<(&UiSlot, &mut Text), (With<UiItemSlotButton>, Changed<UiSlot>)>,
) {
    for (slot, mut text) in &mut ui_slots {
        text.0 = slot.amount.to_string();
    }
}

fn ui_world_time_text(
    mut ui_time_text: Query<&mut Text2d, With<UiWorldTime>>,
    world: Res<WorldSettings>
) {
    if let Some(mut text) = ui_time_text.iter_mut().next() {
        let world_time = format!("{:?}:00 {:?}", world.actual_hour as i32, world.meridiem);
        
        text.0 = world_time.to_string();
    };
}

fn highlight_slot_selected(
    item_selected: Res<ItemSelected>,
    mut ui_slots: Query<(&mut BackgroundColor, Entity),With<UiItemSlotButton>>
) {
    for (mut bgc, entity) in &mut ui_slots {
        if item_selected.ui_entity == entity {
            bgc.0 = Color::srgb(0.7, 0.7, 0.7);
        }else {
            bgc.0 = Color::srgb(RGBINVSLOT.0,RGBINVSLOT.1,RGBINVSLOT.2);
        }
    }
}

fn reset_player_item_selected(
    mut player_selected_item: ResMut<ItemSelected>,
    ui_slots: Query<(&UiSlot,Entity),With<UiItemSlotButton>>
) {
    let entity = ui_slots.iter().find(|s|(s.1 == player_selected_item.ui_entity));

    match entity {
        Option::Some(_) => {},
        Option::None => { player_selected_item.selected = ItemType::None; player_selected_item.ui_entity = Entity::from_raw(0); }
    }
}

fn reset_selected_item(
    mut cmm: Commands,
    mut dyn_ui: ResMut<DynamicUi>,
    input_key: Res<ButtonInput<KeyCode>>,
    input_mouse: Res<ButtonInput<MouseButton>>,
    mut player_selected_item: ResMut<ItemSelected>
) {
    if input_mouse.just_pressed(MouseButton::Right) || input_key.just_pressed(KeyCode::Escape) {
        player_selected_item.selected = ItemType::None; player_selected_item.ui_entity = Entity::from_raw(0); 
        
        if dyn_ui.selected != ItemType::None {
            dyn_ui.open = false; // no needed!
            dyn_ui.selected = ItemType::None;
            cmm.entity(dyn_ui.actual_ui_entity).despawn();
        }
    }
}

// DYNAMIC UI

fn dyn_ui_selection(
    mut dyn_ui: ResMut<DynamicUi>,
    item_selected: Res<ItemSelected>,
    mouse_position: Res<MyWorldCoords>,
    buildings_tuple: Res<BuildingTuple>,
    building_coords: Res<BuildingCoords>,
    input: Res<ButtonInput<MouseButton>>,
    world_entities: Query<(&Transform, Entity), With<HasDynamicMenu>>
) {
    if input.just_pressed(MouseButton::Left) && item_selected.selected == ItemType::None && 
        building_coords.data.contains(&(mouse_position.0.x as i32, mouse_position.0.y as i32)) {

        if let Some(building) = buildings_tuple.data.iter().find(|bt|bt.0 == (mouse_position.0.x as i32, mouse_position.0.y as i32)) {
            
            let menu_selected = match building.1 {
                ItemType::Dirt => { (ItemType::Dirt, true, 2) }
                _ => { (ItemType::None, false, 0) }
            };

            (dyn_ui.selected, dyn_ui.open, dyn_ui.button_count) = menu_selected;

            // search the entity of the building and store it to later activate or change the building from the dyn menu 
            if let Some(ent) = world_entities.iter().find(|we| we.0.translation.x == mouse_position.0.x && we.0.translation.y == mouse_position.0.y) {
                dyn_ui.world_entity = ent.1;
            };

            println!("{:?}", dyn_ui);
        }
    }
}

fn display_dyn_ui_selected(
    mut cmm: Commands,
    mut dyn_ui: ResMut<DynamicUi>,
) {
    const INVENTORY_ROW_GAP: f32 = 10.0;
    const BUTTON_SLOT_SIZE: f32 = 75.0;

    if dyn_ui.open {
        match dyn_ui.selected {
            ItemType::Dirt => {
                cmm.entity(dyn_ui.parent_ui_entity).with_children(|dyn_menu| {
                    dyn_ui.actual_ui_entity = dyn_menu.spawn((
                        Node {
                            width: Val::Px(BUTTON_SLOT_SIZE * dyn_ui.button_count as f32),
                            height: Val::Px(60.),
                            display: Display::Flex,
                            column_gap: Val::Px(INVENTORY_ROW_GAP),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        }, BackgroundColor(Color::srgb(0.3,0.3,0.3))
                    )).with_children(|slots| {
                        for i in 0..dyn_ui.button_count {
                            println!("{:?}", i);
                            slots.spawn((
                                Node {
                                    width: Val::Px(40.),
                                    height: Val::Px(40.),
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.26,0.26,0.26)),
                                Button,
                                match i {
                                    0 => { DynamicButtonId::ButtonOne },
                                    1 => { DynamicButtonId::ButtonTwo },
                                    2 => { DynamicButtonId::ButtonThree },
                                    3 => { DynamicButtonId::ButtonFour },
                                    4 => { DynamicButtonId::ButtonFive },
                                    5 => { DynamicButtonId::ButtonSix },
                                    _ => { DynamicButtonId::ButtonOne }
                                }
                            ));
                        }
                    }).id();
                });
            },
            _ => {}
        }

        dyn_ui.open = false;
    }
}

// actions of the actual dynamic menu displayed
fn dynamic_menu_actions( // rewrite pending
    dyn_ui: Res<DynamicUi>,
    input: Res<ButtonInput<MouseButton>>,
    mut crops: Query<(&mut PreparedDirtData, Entity), With<PreparedDirtData>>,
    dyn_button: Query<(&Interaction,&DynamicButtonId), (With<DynamicButtonId>, Without<UiItemSlotButton>)>
) {
    if input.just_pressed(MouseButton::Left) {
        match dyn_ui.selected {
            ItemType::Dirt => {
                for (interaction, id) in &dyn_button {
                    match interaction {
                        Interaction::Pressed => {
                            println!("presed option {:?}", id);
                            match id {
                                DynamicButtonId::ButtonOne => {
                                    if let Some(mut crop) = crops.iter_mut().find(|c|c.1 == dyn_ui.world_entity) {
                                        crop.0.crop_type_selected = true;
                                    };
                                },
                                DynamicButtonId::ButtonTwo => {},
                                _ => {}           
                            }
                        },
                        _ => {  }
                    }
                }
            }
            _ => {  }
        }
    }
}