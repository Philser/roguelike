use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    components::{
        combat_stats::CombatStats,
        item::{HealthPotion, ItemType},
        position::Position,
    },
    player::Player,
    user_interface::{ActionLog, ActionLogText, HealthBar, HealthText},
    utils::input_utils::get_movement_input,
    GameState,
};

use super::components::{
    Inventory, InventoryCursor, InventoryError, InventoryUIRoot, InventoryUISlot,
    InventoryUISlotFrame, WantsToPickupItem,
};
use lazy_static::lazy_static;

const UNKNOWN_ITEM_COLOR: Color = Color::PINK; // study the greats: Source Engine edition

lazy_static! {
    static ref EMPTY_SLOT_COLOR: Color = Color::rgb_u8(145, 145, 145);
    static ref ITEM_TYPE_COLOR_MAP: HashMap<ItemType, Color> = HashMap::from([
        (ItemType::HealthPotion, Color::GREEN),
        (ItemType::Nothing, Color::rgb_u8(145, 145, 145))
    ]);
}

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

        if let Err(err) = player_inv.add_item((
            pickup_attempt.item.item_type.clone(),
            Some(pickup_attempt.entity),
        )) {
            if err == InventoryError::InventoryFull {
                action_log.entries.push("Inventory is full!".to_owned());
                return;
            }

            todo!("Unimplemented error case!")
        }

        action_log
            .entries
            .push(format!("Picked up {}", pickup_attempt.item.item_type));

        commands.entity(pickup_attempt_entity).despawn();
    }
}

/// System processing player input while in the inventory. Responsible for moving the cursor for the selected
/// item slot, using items and closing the inventory.
/// // TODO: This gets too large, find a way to split this up
pub fn user_input_handler(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
    mut commands: Commands,
    inventory_ui_root_query: Query<Entity, With<InventoryUIRoot>>,
    mut cursor_query: Query<(Entity, &mut InventoryCursor)>,
    mut inventory_query: Query<&mut Inventory>,
    health_pots_query: Query<(&HealthPotion, Entity)>,
    ui_slots_query: Query<Entity, With<UISlots>>,
    player_stats_query: Query<&mut CombatStats, With<Player>>,
    healthtext_query: Query<&mut Text, (With<HealthText>, Without<ActionLogText>)>,
    healthbar_query: Query<&mut Style, With<HealthBar>>,
) {
    let key_press = keyboard_input.clone();
    if key_press.get_just_pressed().len() == 0 {
        return;
    }
    keyboard_input.clear();

    let inventory_ui_root = inventory_ui_root_query
        .get_single()
        .expect("No or too many inventory UIs found");

    let (cursor_entity, mut inventory_cursor) = cursor_query
        .get_single_mut()
        .expect("while fetching cursor");

    let mut inventory = inventory_query
        .get_single_mut()
        .expect("no or more than one inventory");

    let input = get_movement_input(&key_press);
    if input.received_movement_input() {
        inventory_cursor.move_cursor(input.y);
    }

    if key_press.just_pressed(KeyCode::E) {
        use_item(
            &mut commands,
            &inventory_cursor,
            &mut inventory,
            health_pots_query,
            player_stats_query,
            healthtext_query,
            healthbar_query,
        );
    }

    if key_press.just_pressed(KeyCode::I) || key_press.just_pressed(KeyCode::Escape) {
        let ui_slots_entity = ui_slots_query
            .get_single()
            .expect("while querying in user_input_handler");
        exit_inventory(
            &mut commands,
            inventory_ui_root,
            cursor_entity,
            ui_slots_entity,
            &mut app_state,
        );
        return;
    }

    app_state
        .set(GameState::RenderInventory)
        .expect("failed to set game state in inventory.user_input_handler");
}

/// System to create an InventoryCursor object and the UI Entities
pub fn inventory_setup(
    mut commands: Commands,
    mut app_state: ResMut<State<GameState>>,
    player_inventory_query: Query<&Inventory, With<Player>>,
) {
    let player_inventory = player_inventory_query
        .get_single()
        .expect("while retrieving single player inventory");

    commands
        .spawn()
        .insert(InventoryCursor::new(0, player_inventory.inventory_size));

    let mut commands_builder = commands.spawn_bundle(get_ui_root_bundle());
    commands_builder.insert(InventoryUIRoot {});

    let mut ui_slots = UISlots { slots: vec![] };
    commands_builder.with_children(|parent| {
        ui_slots = build_ui_slots(parent, player_inventory);
    });

    commands.spawn().insert(ui_slots);

    app_state
        .set(GameState::RenderInventory)
        .expect("failed to set game state in inventory_setup");
}

/// System that creates the UI elements when the user enters the inventory.
/// Changes app state to AwaitingInventoryInput when done.
pub fn inventory_renderer(
    mut app_state: ResMut<State<GameState>>,
    player_inventory_query: Query<&Inventory, With<Player>>,
    inventory_cursor_query: Query<&InventoryCursor>,
    ui_slots_query: Query<&UISlots>,
    slot_color_query: Query<(
        &mut UiColor,
        Entity,
        With<InventoryUISlot>,
        Without<InventoryUISlotFrame>,
    )>,
    cursor_color_query: Query<(
        &mut UiColor,
        Entity,
        With<InventoryUISlotFrame>,
        Without<InventoryUISlot>,
    )>,
) {
    let player_inventory = player_inventory_query
        .get_single()
        .expect("while retrieving single player inventory");

    let cursor = inventory_cursor_query
        .get_single()
        .expect("while retrieving cursor in inventory_renderer");

    let ui_slots = ui_slots_query.get_single().unwrap();
    render_cursor(ui_slots, cursor, cursor_color_query);

    render_inventory_slots(ui_slots, player_inventory, slot_color_query);

    app_state
        .set(GameState::AwaitingInventoryInput)
        .expect("failed to set game state in inventory_renderer");
}

fn render_cursor(
    ui_slots: &UISlots,
    cursor: &InventoryCursor,
    mut cursor_color_query: Query<(
        &mut UiColor,
        Entity,
        With<InventoryUISlotFrame>,
        Without<InventoryUISlot>,
    )>,
) {
    let mut cursor_entity = Entity::from_raw(0);
    for slot in &ui_slots.slots {
        if slot.inventory_pos == cursor.cursor_position {
            cursor_entity = slot.cursor_slot.clone();
        }
    }

    for (mut color, entity, _, _) in cursor_color_query.iter_mut() {
        if entity == cursor_entity {
            color.0 = Color::WHITE;
        } else {
            color.0 = Color::GRAY;
        }
    }
}

fn render_inventory_slots(
    ui_slots: &UISlots,
    player_inventory: &Inventory,
    mut slot_color_query: Query<(
        &mut UiColor,
        Entity,
        With<InventoryUISlot>,
        Without<InventoryUISlotFrame>,
    )>,
) {
    let mut entity_map: HashMap<Entity, usize> = HashMap::new();
    for slot in &ui_slots.slots {
        entity_map.insert(slot.item_slot, slot.inventory_pos);
    }

    for (mut color, entity, _, _) in slot_color_query.iter_mut() {
        if let Some(pos) = entity_map.get(&entity) {
            let inventory_content = &player_inventory.items[*pos];
            color.0 = get_item_color(&inventory_content.0);
        }
    }
}

fn exit_inventory(
    commands: &mut Commands,
    inventory_ui_root: Entity,
    cursor: Entity,
    ui_slots_entity: Entity,
    app_state: &mut ResMut<State<GameState>>,
) {
    commands.entity(inventory_ui_root).despawn_recursive();
    commands.entity(cursor).despawn();
    commands.entity(ui_slots_entity).despawn();

    app_state
        .set(GameState::AwaitingActionInput)
        .expect("Couldn't go back to AwaitingActionInput");
}

pub struct UISlot {
    pub cursor_slot: Entity,
    pub item_slot: Entity,
    pub inventory_pos: usize,
}

fn build_ui_slot(parent: &mut ChildBuilder, y: f32, inventory_pos: usize) -> UISlot {
    // TODO: Make size of slots depend on size of inventory
    let slot_height_px = 60.0;
    let slot_width_px = 60.0;
    let gap_size_px = 15.0;

    // Spawn cursor frame, surrounding item frame
    let mut cursor_entity_comm = parent.spawn_bundle(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                left: Val::Px(slot_width_px + 1.0 * gap_size_px),
                bottom: Val::Px(y * slot_height_px + (y + 1.0) * gap_size_px),
                ..Default::default()
            },
            size: Size::new(Val::Px(slot_width_px), Val::Px(slot_height_px)),
            ..default()
        },
        color: Color::WHITE.into(),
        ..default()
    });
    cursor_entity_comm.insert(InventoryUISlotFrame {});

    // Spawn item frame
    let mut item_slot = Entity::from_raw(0);
    cursor_entity_comm.with_children(|par| {
        let mut item_slot_comm = par.spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Relative,
                position: Rect {
                    left: Val::Percent(5.0),
                    bottom: Val::Percent(5.0),
                    ..Default::default()
                },
                size: Size::new(Val::Percent(90.0), Val::Percent(90.0)),
                ..default()
            },
            color: Color::GRAY.into(),
            ..default()
        });
        item_slot_comm.insert(InventoryUISlot {});
        item_slot = item_slot_comm.id();
    });

    return UISlot {
        cursor_slot: cursor_entity_comm.id(),
        item_slot,
        inventory_pos,
    };
}

#[derive(Component)]
pub struct UISlots {
    pub slots: Vec<UISlot>,
}

fn build_ui_slots(parent: &mut ChildBuilder, inventory: &Inventory) -> UISlots {
    let mut reverse_y = inventory.inventory_size;
    let mut slots = vec![];
    for y in 0..inventory.inventory_size {
        reverse_y -= 1;

        let ui_slot = build_ui_slot(parent, y as f32, reverse_y);
        slots.push(ui_slot);
    }

    UISlots { slots }
}

fn get_item_color(item_type: &ItemType) -> Color {
    ITEM_TYPE_COLOR_MAP
        .get(item_type)
        .unwrap_or(&UNKNOWN_ITEM_COLOR)
        .clone()
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

fn use_item(
    commands: &mut Commands,
    inventory_cursor: &InventoryCursor,
    inventory: &mut Inventory,
    health_pots_query: Query<(&HealthPotion, Entity)>,
    player_stats_query: Query<&mut CombatStats, With<Player>>,
    healthtext_query: Query<&mut Text, (With<HealthText>, Without<ActionLogText>)>,
    healthbar_query: Query<&mut Style, With<HealthBar>>,
) {
    let item = &inventory.items[inventory_cursor.cursor_position];

    match item.0 {
        ItemType::HealthPotion => use_health_pot(
            item.1.unwrap(),
            commands,
            inventory_cursor,
            inventory,
            health_pots_query,
            player_stats_query,
            healthtext_query,
            healthbar_query,
        ),
        ItemType::Nothing => {
            return;
        }
    }
}

fn use_health_pot(
    item_entity: Entity,
    commands: &mut Commands,
    inventory_cursor: &InventoryCursor,
    inventory: &mut Inventory,
    health_pots_query: Query<(&HealthPotion, Entity)>,
    mut player_stats_query: Query<&mut CombatStats, With<Player>>,
    mut healthtext_query: Query<&mut Text, (With<HealthText>, Without<ActionLogText>)>,
    mut healthbar_query: Query<&mut Style, With<HealthBar>>,
) {
    let mut player_stats = player_stats_query.get_single_mut().expect("in use_item");

    for (health_pot, entity) in health_pots_query.iter() {
        if entity == item_entity {
            player_stats.heal(health_pot.heal_amount);
            commands.entity(item_entity).despawn();
        }
    }

    let mut healthbar = healthbar_query
        .get_single_mut()
        .expect("Found more or less than exactly one Healthbar entity while rendering UI");

    healthbar.size.width = Val::Px(player_stats.hp as f32 * 3.0);

    let mut healthtext = healthtext_query
        .get_single_mut()
        .expect("Found more or less than exactly one Healthtext entity while rendering UI");

    // We only care for the first section
    healthtext.sections[0].value = format!("{}/{}", player_stats.hp, player_stats.max_hp);

    inventory.remove_item(inventory_cursor.cursor_position);
}
