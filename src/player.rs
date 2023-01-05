use bevy::prelude::*;

use crate::{
    components::position::Position,
    components::{
        combat_stats::CombatStats,
        damage::DamageTracker,
        damage::SufferDamage,
        item::{Item, ItemName, UNKNOWN_ITEM_NAME},
        user_input::UserInput,
    },
    inventory::components::WantsToPickupItem,
    map::game_map::GameMap,
    user_interface::ActionLog,
    utils::{input_utils::get_movement_input, render::map_pos_to_screen_pos},
    viewshed::Viewshed,
    GameConfig, GameState,
};

pub const PLAYER_FOV: i32 = 10;

pub const PLAYER_TURN_LABEL: &str = "player_turn";
pub struct PlayerPlugin {}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::PlayerTurn)
                .with_system(player_turn.label(PLAYER_TURN_LABEL)),
        );
        app.add_system(player_input.label("await_input"));
    }
}

/*
* Note:  The creation of the player entity is done in GameMapPlugin
*/
#[derive(Component)]
pub struct Player {}

fn player_input(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut user_input_res: ResMut<UserInput>,
    mut app_state: ResMut<State<GameState>>,
    items_query: Query<(Entity, &Position, Option<&ItemName>), With<Item>>,
    player_query: Query<&Position, With<Player>>,
    mut commands: Commands,
) {
    if *app_state.current() != GameState::AwaitingActionInput {
        return;
    }

    let user_input = get_movement_input(&keyboard_input);
    let mut received_input = user_input.received_movement_input();

    if keyboard_input.just_pressed(KeyCode::G) {
        let player_pos = player_query
            .get_single()
            .expect("Player does not exist or has no position");

        for (entity, item_pos, item_name) in items_query.iter() {
            if player_pos == item_pos {
                let name;
                if item_name.is_none() {
                    name = UNKNOWN_ITEM_NAME.to_owned();
                } else {
                    name = item_name.unwrap().name.clone()
                }

                commands.spawn_empty().insert(WantsToPickupItem {
                    entity,
                    item_name: name,
                });
                received_input = true;
                break;
            }
        }
    }

    if keyboard_input.just_pressed(KeyCode::I) {
        app_state
            .set(GameState::SetupInventoryScreen)
            .expect("failed to set game state to InventoryMenu");
    } else if keyboard_input.just_pressed(KeyCode::Escape) {
        app_state
            .push(GameState::MainMenu)
            .expect("failed to set game state to InventoryMenu");
    } else if received_input {
        user_input_res.x = user_input.x;
        user_input_res.y = user_input.y;

        app_state
            .set(GameState::PlayerTurn)
            .expect("failed to set game state in player_input")
    }

    keyboard_input.clear();
}

/// Moves the player if no obstacle is in the way or tries to fight the obstacle, if fightable.
/// Is only called if game state is in `GameState::PlayerTurn`.
/// At the end of the player turn, set the game to `GameState::MonsterTurn`.
fn player_turn(
    mut player_query: Query<(
        Entity,
        &mut Transform,
        &mut Position,
        &mut Viewshed,
        With<Player>,
    )>,
    mut combattable_query: Query<&mut CombatStats>,
    mut map: ResMut<GameMap>,
    mut damage_tracker: ResMut<DamageTracker>,
    mut app_state: ResMut<State<GameState>>,
    mut user_input_res: ResMut<UserInput>,
    mut action_log: ResMut<ActionLog>,
    game_config: Res<GameConfig>,
) {
    if let Ok((player_entity, mut player_tf, mut player_pos, mut viewshed, _)) =
        player_query.get_single_mut()
    {
        if user_input_res.x != 0 || user_input_res.y != 0 {
            // Check for collisions
            let new_x = player_pos.x + user_input_res.x;
            let new_y = player_pos.y + user_input_res.y;

            let new_pos = Position { x: new_x, y: new_y };

            if map.is_blocked(&new_pos) {
                if let Some(entity) = map.tile_content.get(&new_pos) {
                    if let Ok(combattable) =
                        combattable_query.get_many_mut([*entity, player_entity])
                    {
                        // We found something to hit here
                        let player_power = combattable[1].power;
                        SufferDamage::add_damage(
                            &mut damage_tracker,
                            *entity,
                            player_power,
                            action_log.as_mut(),
                            true,
                        );
                        bevy::log::info!(
                            "Monster has been hit with {} and has {} hp left",
                            player_power,
                            combattable[0].hp - player_power
                        );
                    } else {
                        bevy::log::warn!(
                            "Could not find combattable component of at least one entity"
                        )
                    }
                }
            } else {
                // unblock old position
                map.remove_blocked(&player_pos);
                map.remove_tile_content(&player_pos);

                // block new position
                map.set_blocked(new_pos);
                map.set_tile_content(player_pos.clone(), player_entity);

                player_pos.x = new_x;
                player_pos.y = new_y;

                player_tf.translation = map_pos_to_screen_pos(
                    &player_pos,
                    game_config.tile_properties.player_z,
                    game_config.tile_properties.tile_size,
                    &game_config.screen_dimensions,
                );

                viewshed.dirty = true;

                user_input_res.x = 0;
                user_input_res.y = 0;
            }
        }
        app_state
            .set(GameState::MonsterTurn)
            .expect("failed to set game state in try_move_player");
    }
}
