use bevy::{prelude::*, ui};

use crate::{
    components::{inventory::Inventory, item::Item, position::Position, user_input::UserInput},
    player::Player,
    user_interface::{ActionLog, UIFont},
    utils::input_utils::get_WASD_movement,
    GameState,
};

pub struct InventorySystemPlugin {}

impl Plugin for InventorySystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::PlayerTurn).with_system(pickup_handler));
        app.add_system_set(
            SystemSet::on_update(GameState::AwaitingInventoryInput).with_system(input_handler),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::RenderInventory).with_system(render_inventory),
        );
    }
}

const INVENTORY_SLOTS_WIDTH: i32 = 4;
const INVENTORY_SLOTS_HEIGHT: i32 = 4;

#[derive(Component)]
pub struct InventoryUIFrame {}

#[derive(Component)]
pub struct InventoryUISlot {}

#[derive(Component)]
pub struct InventoryCursor {
    pub cursor_position: Position,
    pub ui_item_slots: Vec<Vec<Entity>>,
}

impl InventoryCursor {
    pub fn move_cursor(&mut self, x: i32, y: i32) {
        let new_x = self.cursor_position.x + x;
        let new_y = self.cursor_position.y + y;

        if new_y >= 0 && self.ui_item_slots.len() > new_y as usize {
            self.cursor_position.y = new_y;
        }

        if new_x >= 0 && self.ui_item_slots[self.cursor_position.y as usize].len() > new_x as usize
        {
            self.cursor_position.x = new_x;
        }
    }
}

#[derive(Component)]
pub struct WantsToPickupItem {
    pub entity: Entity,
    pub item: Item,
}

fn pickup_handler(
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

fn input_handler(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<GameState>>,
    mut commands: Commands,
    inventory_ui_query: Query<Entity, With<InventoryUIFrame>>,
    mut inventory_slots_query: Query<(Entity, &mut UiColor), With<InventoryUISlot>>,
    mut cursor_query: Query<(Entity, &mut InventoryCursor)>,
) {
    let inv_entity = inventory_ui_query
        .get_single()
        .expect("No or too many inventory UIs found");

    let (cursor_entity, mut inventory_cursor) = cursor_query
        .get_single_mut()
        .expect("while fetching cursor");

    let input = get_WASD_movement(&keyboard_input);
    let received_input = input.received_input();

    if keyboard_input.just_pressed(KeyCode::I) || keyboard_input.just_pressed(KeyCode::Escape) {
        commands.entity(inv_entity).despawn_recursive();
        commands.entity(cursor_entity).despawn();

        app_state
            .set(GameState::AwaitingActionInput)
            .expect("Couldn't go back to AwaitingActionInput");
    } else if received_input {
        // TODO: Change active inventory element
        let curr_entity = inventory_cursor.ui_item_slots
            [inventory_cursor.cursor_position.y as usize]
            [inventory_cursor.cursor_position.x as usize];

        inventory_cursor.move_cursor(input.x, input.y);

        let new_entity = inventory_cursor.ui_item_slots
            [inventory_cursor.cursor_position.y as usize]
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

    keyboard_input.clear();
}

fn render_inventory(
    mut commands: Commands,
    player_query: Query<&Inventory, With<Player>>,
    default_font: Res<UIFont>,
    mut app_state: ResMut<State<GameState>>,
) {
    let mut commands_builder = commands.spawn_bundle(NodeBundle {
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
    });
    commands_builder.insert(InventoryUIFrame {});

    // TODO: Make size of slots depend on size of inventory
    let slot_height_px = 60.0;
    let slot_width_px = 60.0;
    let gap_size_px = 15.0;
    let mut ui_item_slots: Vec<Vec<Entity>> = vec![];
    commands_builder.with_children(|parent| {
        for i in 0..INVENTORY_SLOTS_HEIGHT {
            let y = i as f32;
            ui_item_slots.push(vec![]);
            for j in 0..INVENTORY_SLOTS_WIDTH {
                let x = j as f32;

                let mut color = Color::BLACK;
                if i == 0 && j == 0 {
                    color = Color::WHITE;
                }

                let slot_entity = parent
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
                        color: color.into(),
                        ..default()
                    })
                    .insert(InventoryUISlot {})
                    .id();
                ui_item_slots[i as usize].push(slot_entity);
            }
        }
    });

    commands.spawn().insert(InventoryCursor {
        cursor_position: Position { x: 0, y: 0 },
        ui_item_slots,
    });

    app_state
        .set(GameState::AwaitingInventoryInput)
        .expect("failed to set game state in map.setup()");
}
