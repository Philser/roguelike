use bevy::prelude::*;

use crate::{
    components::position::Position,
    player::Player,
    user_interface::ActionLog,
    utils::input_utils::{get_movement_input, MovementInput},
    GameState,
};

use super::components::{
    Inventory, InventoryCursor, InventoryUIRoot, InventoryUISlot, WantsToPickupItem,
};

const INVENTORY_SLOTS_WIDTH: i32 = 4;
const INVENTORY_SLOTS_HEIGHT: i32 = 4;

/// System for processing a pickup action by the user. Removes the item in question from the map
/// and adds it to the player inventory.
pub fn pickup_handler(
    mut commands: Commands,
    pickup_query: Query<(Entity, &WantsToPickupItem)>,
    mut player_inventory_query: Query<&mut Inventory, With<Player>>,
    mut action_log: ResMut<ActionLog>,
) {
    let mut player_inv = player_inventory_query
        .get_single_mut()
        .expect("We don't have exactly one inventory!!11");

    for (pickup_attempt_entity, pickup_attempt) in pickup_query.iter() {
        // remove item from map
        commands.entity(pickup_attempt.entity).remove::<Sprite>();
        commands.entity(pickup_attempt.entity).remove::<Transform>();
        commands.entity(pickup_attempt.entity).remove::<Position>();

        player_inv.add(&pickup_attempt.item.item_type, pickup_attempt.entity);

        action_log
            .entries
            .push(format!("Picked up {}", pickup_attempt.item.item_type));

        commands.entity(pickup_attempt_entity).despawn();
    }
}

/// System processing player input while in the inventory. Responsible for moving the cursor for the selected
/// item slot, using items and closing the inventory.
pub fn user_input_handler(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
    mut commands: Commands,
    inventory_ui_query: Query<Entity, With<InventoryUIRoot>>,
    inventory_slots_query: Query<(Entity, &mut UiColor), With<InventoryUISlot>>,
    mut cursor_query: Query<(Entity, &mut InventoryCursor)>,
) {
    let key_press = keyboard_input.clone();
    keyboard_input.clear();

    let inventory_ui_root = inventory_ui_query
        .get_single()
        .expect("No or too many inventory UIs found");

    let (cursor_entity, mut inventory_cursor) = cursor_query
        .get_single_mut()
        .expect("while fetching cursor");

    let input = get_movement_input(&key_press);
    if input.received_movement_input() {
        move_cursor(&mut inventory_cursor, &input, inventory_slots_query);
    }

    if key_press.just_pressed(KeyCode::I) || key_press.just_pressed(KeyCode::Escape) {
        exit_inventory(
            &mut commands,
            inventory_ui_root,
            cursor_entity,
            &mut app_state,
        );
    }
}

/// System that creates the UI elements when the user enters the inventory.
/// Changes app state to AwaitingInventoryInput when done.
pub fn inventory_renderer(mut commands: Commands, mut app_state: ResMut<State<GameState>>) {
    let mut commands_builder = commands.spawn_bundle(get_ui_root_bundle());
    commands_builder.insert(InventoryUIRoot {});

    let mut ui_item_slots: Vec<Vec<Entity>> = vec![];
    commands_builder.with_children(|parent| build_ui_slots(parent, &mut ui_item_slots));

    commands.spawn().insert(InventoryCursor {
        cursor_position: Position { x: 0, y: 0 },
        ui_item_slots,
    });

    app_state
        .set(GameState::AwaitingInventoryInput)
        .expect("failed to set game state in map.setup()");
}

fn move_cursor(
    inventory_cursor: &mut InventoryCursor,
    input: &MovementInput,
    mut inventory_slots_query: Query<(Entity, &mut UiColor), With<InventoryUISlot>>,
) {
    let curr_entity = inventory_cursor.ui_item_slots[inventory_cursor.cursor_position.y as usize]
        [inventory_cursor.cursor_position.x as usize];

    inventory_cursor.move_cursor(input.x, input.y);

    let new_entity = inventory_cursor.ui_item_slots[inventory_cursor.cursor_position.y as usize]
        [inventory_cursor.cursor_position.x as usize];

    if curr_entity != new_entity {
        for (entity, mut color) in inventory_slots_query.iter_mut() {
            if entity == curr_entity {
                color.0 = Color::BLACK
            } else if entity == new_entity {
                color.0 = Color::WHITE
            }
        }
    }
}

fn exit_inventory(
    commands: &mut Commands,
    inventory_ui_root: Entity,
    cursor: Entity,
    app_state: &mut ResMut<State<GameState>>,
) {
    commands.entity(inventory_ui_root).despawn_recursive();
    commands.entity(cursor).despawn();

    app_state
        .set(GameState::AwaitingActionInput)
        .expect("Couldn't go back to AwaitingActionInput");
}

fn build_ui_slot(parent: &mut ChildBuilder, x: f32, y: f32, color: UiColor) -> Entity {
    // TODO: Make size of slots depend on size of inventory
    let slot_height_px = 60.0;
    let slot_width_px = 60.0;
    let gap_size_px = 15.0;

    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(slot_width_px), Val::Px(slot_height_px)),
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(x * slot_width_px + (x + 1.0) * gap_size_px),
                    bottom: Val::Px(y * slot_height_px + (y + 1.0) * gap_size_px),
                    ..Default::default()
                },
                ..default()
            },
            color: color,
            ..default()
        })
        .insert(InventoryUISlot {})
        .id()
}

fn build_ui_slots(parent: &mut ChildBuilder, ui_item_slots: &mut Vec<Vec<Entity>>) {
    for i in 0..INVENTORY_SLOTS_HEIGHT {
        let y = i as f32;
        ui_item_slots.push(vec![]);
        for j in 0..INVENTORY_SLOTS_WIDTH {
            let x = j as f32;

            let mut color = Color::BLACK;
            if i == 0 && j == 0 {
                color = Color::WHITE;
            }

            ui_item_slots[i as usize].push(build_ui_slot(parent, x, y, color.into()));
        }
    }
}

fn get_ui_root_bundle() -> NodeBundle {
    NodeBundle {
        style: Style {
            // TODO: Parameterize inventory size
            // TODO: Use Val::Px
            size: Size::new(Val::Percent(40.0), Val::Percent(45.0)),
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Percent(30.0),
                right: Val::Percent(30.0),
                top: Val::Percent(30.0),
                bottom: Val::Percent(30.0),
            },
            ..default()
        },
        color: Color::PURPLE.into(),
        ..default()
    }
}
