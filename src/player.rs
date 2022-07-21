use bevy::prelude::*;

use crate::{
    components::{
        suffer_damage::DamageTracker, suffer_damage::SufferDamage, user_input::UserInput,
        CombatStats::CombatStats,
    },
    map::GameMap,
    position::Position,
    user_interface::ActionLog,
    utils::render::map_pos_to_screen_pos,
    viewshed::Viewshed,
    GameState, PLAYER_Z, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE,
};

pub const PLAYER_STARTING_HEALTH: i32 = 100;
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
    keyboard_input: Res<Input<KeyCode>>,
    mut user_input_res: ResMut<UserInput>,
    mut app_state: ResMut<State<GameState>>,
) {
    if *app_state.current() != GameState::AwaitingInput {
        return;
    }

    let mut tried_move = false;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    if keyboard_input.just_pressed(KeyCode::A) {
        x = -1;
        y = 0;
        tried_move = true;
    }
    if keyboard_input.just_pressed(KeyCode::D) {
        x = 1;
        y = 0;
        tried_move = true;
    }
    if keyboard_input.just_pressed(KeyCode::W) {
        x = 0;
        y = 1;
        tried_move = true;
    }
    if keyboard_input.just_pressed(KeyCode::S) {
        x = 0;
        y = -1;
        tried_move = true;
    }

    if tried_move {
        user_input_res.x = x;
        user_input_res.y = y;

        app_state
            .set(GameState::PlayerTurn)
            .expect("failed to set game state in player_input")
    }
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
                    PLAYER_Z,
                    TILE_SIZE,
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                );

                viewshed.dirty = true;

                user_input_res.x = 0;
                user_input_res.y = 0;
            }

            app_state
                .set(GameState::MonsterTurn)
                .expect("failed to set game state in try_move_player");
        }
    }
}
