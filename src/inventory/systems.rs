use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    components::{
        combat_stats::CombatStats,
        consumable::Consumable,
        item::{Heals, Item, Ranged},
        position::Position,
    },
    player::Player,
    user_interface::{ActionLog, ActionLogText, HealthBar, HealthText, TargetingModeContext},
    utils::input_utils::get_movement_input,
    GameState,
};

use super::components::{
    Inventory, InventoryCursor, InventoryError, InventoryUIRoot, InventoryUISlot,
    InventoryUISlotFrame, WantsToPickupItem,
};

const UNKNOWN_ITEM_COLOR: Color = Color::YELLOW;
const EMPTY_SLOT_COLOR: Color = Color::GRAY;

/// System for processing a pickup action by the user. Removes the item in question from the map
/// and adds it to the player inventory.
pub fn pickup_handler(
    mut commands: Commands,
    pickup_query: Query<(Entity, &WantsToPickupItem)>,
    mut visiblity_query: Query<&mut Visibility, With<Item>>,
    mut player_inventory_query: Query<&mut Inventory, With<Player>>,
    mut action_log: ResMut<ActionLog>,
) {
    let mut player_inv = player_inventory_query
        .get_single_mut()
        .expect("We don't have exactly one inventory!!11");

    for (pickup_attempt_entity, pickup_attempt) in pickup_query.iter() {
        match visiblity_query.get_mut(pickup_attempt.entity) {
            Ok(mut vis) => vis.is_visible = false,
            Err(e) => bevy::log::error!("{}", e),
        }
        // remove item from map
        commands.entity(pickup_attempt.entity).remove::<Transform>();
        commands.entity(pickup_attempt.entity).remove::<Position>();

        if let Err(err) = player_inv.add_item(pickup_attempt.entity) {
            if err == InventoryError::InventoryFull {
                action_log.entries.push("Inventory is full!".to_owned());
                return;
            }

            todo!("Unimplemented error case!")
        }

        action_log
            .entries
            .push(format!("Picked up {}", pickup_attempt.item_name));

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
    ui_slots_query: Query<Entity, With<UISlots>>,
    player_stats_query: Query<&mut CombatStats, With<Player>>,
    healthtext_query: Query<&mut Text, (With<HealthText>, Without<ActionLogText>)>,
    healthbar_query: Query<&mut Style, With<HealthBar>>,
    item_query: Query<(Option<&Heals>, Option<&Consumable>, Option<&Ranged>), With<Item>>,
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

    let mut new_app_state = GameState::RenderInventory;
    if key_press.just_pressed(KeyCode::E) {
        new_app_state = use_item(
            &mut commands,
            &inventory_cursor,
            &mut inventory,
            player_stats_query,
            healthtext_query,
            healthbar_query,
            item_query,
        );
    }

    if key_press.just_pressed(KeyCode::I) || key_press.just_pressed(KeyCode::Escape) {
        new_app_state = GameState::AwaitingActionInput;
    }

    if new_app_state == GameState::AwaitingActionInput || new_app_state == GameState::Targeting {
        let ui_slots_entity = ui_slots_query
            .get_single()
            .expect("while querying in user_input_handler");

        despawn_inventory(
            &mut commands,
            inventory_ui_root,
            cursor_entity,
            ui_slots_entity,
        );
    }

    app_state
        .set(new_app_state)
        .expect("failed to set game state in inventory.use_item");
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
    item_sprite_query: Query<&Sprite, With<Item>>,
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

    render_inventory_slots(
        ui_slots,
        player_inventory,
        slot_color_query,
        item_sprite_query,
    );

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
    item_sprite_query: Query<&Sprite, With<Item>>,
) {
    let mut entity_map: HashMap<Entity, usize> = HashMap::new();
    for slot in &ui_slots.slots {
        entity_map.insert(slot.item_slot, slot.inventory_pos);
    }

    for (mut color, entity, _, _) in slot_color_query.iter_mut() {
        if let Some(pos) = entity_map.get(&entity) {
            if let Some(item_in_inventory) = &player_inventory.items[*pos] {
                match item_sprite_query.get(*item_in_inventory) {
                    Ok(item_sprite) => color.0 = item_sprite.color,
                    Err(e) => {
                        bevy::log::error!("{}", e);
                        color.0 = UNKNOWN_ITEM_COLOR;
                    }
                }
            } else {
                color.0 = EMPTY_SLOT_COLOR
            }
        }
    }
}

fn despawn_inventory(
    commands: &mut Commands,
    inventory_ui_root: Entity,
    cursor: Entity,
    ui_slots_entity: Entity,
) {
    commands.entity(inventory_ui_root).despawn_recursive();
    commands.entity(cursor).despawn();
    commands.entity(ui_slots_entity).despawn();
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
            color: EMPTY_SLOT_COLOR.into(),
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
    player_stats_query: Query<&mut CombatStats, With<Player>>,
    healthtext_query: Query<&mut Text, (With<HealthText>, Without<ActionLogText>)>,
    healthbar_query: Query<&mut Style, With<HealthBar>>,
    item_query: Query<(Option<&Heals>, Option<&Consumable>, Option<&Ranged>), With<Item>>,
) -> GameState {
    if let Some(item_entity) = &inventory.items[inventory_cursor.cursor_position] {
        match item_query.get(*item_entity) {
            Ok(query) => {
                if let Some(heals) = query.0 {
                    use_health_pot(heals, player_stats_query, healthtext_query, healthbar_query);
                }

                if let Some(_consumable) = query.1 {
                    commands.entity(*item_entity).despawn();
                    inventory.remove_item(inventory_cursor.cursor_position);
                }

                if let Some(ranged) = query.2 {
                    commands.spawn().insert(TargetingModeContext {
                        range: ranged.range,
                    });
                    return GameState::Targeting;
                }
            }
            Err(_) => bevy::log::error!("Unimplemented item behaviour"),
        }
    }

    return GameState::RenderInventory;
}

fn use_health_pot(
    health_pot: &Heals,
    mut player_stats_query: Query<&mut CombatStats, With<Player>>,
    mut healthtext_query: Query<&mut Text, (With<HealthText>, Without<ActionLogText>)>,
    mut healthbar_query: Query<&mut Style, With<HealthBar>>,
) {
    let mut player_stats = player_stats_query.get_single_mut().expect("in use_item");

    player_stats.heal(health_pot.heal_amount);

    let mut healthbar = healthbar_query
        .get_single_mut()
        .expect("Found more or less than exactly one Healthbar entity while rendering UI");

    healthbar.size.width = Val::Px(player_stats.hp as f32 * 3.0);

    let mut healthtext = healthtext_query
        .get_single_mut()
        .expect("Found more or less than exactly one Healthtext entity while rendering UI");

    // We only care for the first section
    healthtext.sections[0].value = format!("{}/{}", player_stats.hp, player_stats.max_hp);
}
