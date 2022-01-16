use bevy::prelude::*;

use crate::{
    map::GameMap, position::Position, viewshed::Viewshed, GameState, PLAYER_Z, SCREEN_HEIGHT,
    SCREEN_WIDTH, TILE_SIZE,
};

pub const PLAYER_STARTING_HEALTH: i32 = 100;
pub const PLAYER_FOV: i32 = 10;
pub struct PlayerPlugin {}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(try_move_player.system().label("player_movement").after("map_indexer"));
    }
}

/*
* Note:  The creation of the player entity is done in GameMapPlugin
*/

pub struct Player {}

/// Listens for keyboard input and moves the player if no obstacle is in the way.
/// If the player moves, the game state is set to `GameState::GameRunning`.
/// Else, the game state is set to `GameState::GameRunning`
fn try_move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Position, &mut Viewshed, With<Player>)>,
    map: Res<GameMap>,
    mut app_state: ResMut<State<GameState>>,
) {
    if let Ok((mut player_tf, mut player_pos, mut viewshed, _)) = query.single_mut() {
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

            if map.is_blocked(&Position { x: new_x, y: new_y }) {
                return;
            }

            player_pos.x = new_x;
            player_pos.y = new_y;

            app_state
                .push(GameState::PlayerActive)
                .expect("Could not set game to status PlayerActive");

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
