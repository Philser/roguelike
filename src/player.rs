use bevy::prelude::*;

use crate::{
    components::CombatStats::CombatStats, map::GameMap, position::Position, viewshed::Viewshed,
    GameState, PLAYER_Z, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE,
};

pub const PLAYER_STARTING_HEALTH: i32 = 100;
pub const PLAYER_FOV: i32 = 10;
pub struct PlayerPlugin {}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            try_move_player
                .label("player_movement")
                .before("map_indexer"),
        );
    }
}

/*
* Note:  The creation of the player entity is done in GameMapPlugin
*/
#[derive(Component)]
pub struct Player {}

/// Listens for keyboard input and moves the player if no obstacle is in the way.
/// If the player moves, the game state is set to `GameState::GameRunning`.
/// Else, the game state is set to `GameState::GameRunning`
fn try_move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(
        Entity,
        &mut Transform,
        &mut Position,
        &mut Viewshed,
        With<Player>,
    )>,
    mut combattable_query: Query<&mut CombatStats>,
    mut map: ResMut<GameMap>,
    mut app_state: ResMut<State<GameState>>,
) {
    if let Ok((player_entity, mut player_tf, mut player_pos, mut viewshed, _)) =
        player_query.get_single_mut()
    {
        let mut tried_move = false;
        let mut move_coordinates: (i32, i32) = (0, 0);
        if keyboard_input.just_pressed(KeyCode::A) {
            move_coordinates = (-1, 0);
            tried_move = true;
        }
        if keyboard_input.just_pressed(KeyCode::D) {
            move_coordinates = (1, 0);
            tried_move = true;
        }
        if keyboard_input.just_pressed(KeyCode::W) {
            move_coordinates = (0, 1);
            tried_move = true;
        }
        if keyboard_input.just_pressed(KeyCode::S) {
            move_coordinates = (0, -1);
            tried_move = true;
        }

        if tried_move {
            // Check for collisions
            let new_x = player_pos.x + move_coordinates.0;
            let new_y = player_pos.y + move_coordinates.1;

            let new_pos = Position { x: new_x, y: new_y };

            if map.is_blocked(&new_pos) {
                if let Some(entity) = map.tile_content.get(&new_pos) {
                    if let Ok(mut combattable) =
                        combattable_query.get_many_mut([*entity, player_entity])
                    {
                        // We found something to hit here
                        let player_power = combattable[1].power;

                        combattable[0].hurt(player_power);
                        bevy::log::info!(
                            "Monster has been hit with {} and has {} hp left",
                            player_power,
                            combattable[0].hp
                        );
                    } else {
                        bevy::log::warn!(
                            "Could not find combattable component of at least one entity"
                        )
                    }
                }
                return;
            }

            // unblock old position
            map.remove_blocked(&player_pos);
            map.remove_tile_content(&player_pos);

            // block new position
            map.set_blocked(new_pos.clone());
            map.set_tile_content(player_pos.clone(), player_entity.clone());

            player_pos.x = new_x;
            player_pos.y = new_y;

            if app_state.current() != &GameState::PlayerActive {
                // Seems like sometimes the state is not popped fast enough so we end up trying to push
                // the PlayerActive state twice
                app_state
                    .push(GameState::PlayerActive)
                    .expect("Could not set game to status PlayerActive");
            }

            // TODO: Right now I am lazy but this def. needs to
            // be an own function that translates coords to pixels
            // keeping in mind that bevy's pixel coords start from the middle of the screen
            player_tf.translation = Vec3::new(
                new_x as f32 * TILE_SIZE - SCREEN_WIDTH / 2.0,
                new_y as f32 * TILE_SIZE - SCREEN_HEIGHT / 2.0,
                PLAYER_Z,
            );

            viewshed.dirty = true;
        } else if app_state.current() == &GameState::PlayerActive {
            app_state
                .pop()
                .expect("Unexpectedly pop state PlayerActive");
        }
    }
}
