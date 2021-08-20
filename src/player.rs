use bevy::prelude::*;

use crate::{map::Materials, GameState, PLAYER_Z, SCALE, TILE_SIZE};

pub struct PlayerPlugin {}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(try_move_player.system());
    }
}

/*
* Note:  The creation of the player entity is done in GameMapPlugin
*/

pub struct Player {
    pub x: i32,
    pub y: i32,
}

fn try_move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
) {
    let (mut player_tf, mut player) = query
        .single_mut()
        .expect("No player found for running game");

    let mut tried_move = false;
    let mut move_coordinates: (i32, i32) = (0, 0);
    if keyboard_input.just_pressed(KeyCode::A) {
        move_coordinates = (-1 as i32, 0);
        tried_move = true;
    }
    if keyboard_input.just_pressed(KeyCode::D) {
        move_coordinates = (1 as i32, 0);
        tried_move = true;
    }
    if keyboard_input.just_pressed(KeyCode::W) {
        move_coordinates = (0, -1 as i32);
        tried_move = true;
    }
    if keyboard_input.just_pressed(KeyCode::S) {
        move_coordinates = (0, 1 as i32);
        tried_move = true;
    }

    // TODO: Collision check
    player.x += move_coordinates.0;
    player.y -= move_coordinates.1;

    // TODO: Right now I am lazy but this def. needs to
    // be an own function that translates coords to pixels
    // keeping in mind that bevy's pixel coords start from the middle of the screen
    if tried_move {
        player_tf.translation = Vec3::new(
            (player.x as f32 - 10.0) * TILE_SIZE,
            (player.y as f32 - 10.0) * TILE_SIZE,
            PLAYER_Z,
        );
    }
}
