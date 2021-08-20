use bevy::prelude::*;

use crate::{map::GameMap, position::Position, Collidable, PLAYER_Z, TILE_SIZE};

pub const PLAYER_STARTING_HEALTH: i32 = 100;
pub struct PlayerPlugin {}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(try_move_player.system());
    }
}

/*
* Note:  The creation of the player entity is done in GameMapPlugin
*/

pub struct Player {}

fn try_move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Position, With<Player>)>,
    collidables_query: Query<&Collidable>,
) {
    if let Ok((mut player_tf, mut player_pos, _)) = query.single_mut() {
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
            move_coordinates = (0, 1 as i32);
            tried_move = true;
        }
        if keyboard_input.just_pressed(KeyCode::S) {
            move_coordinates = (0, -1 as i32);
            tried_move = true;
        }

        if tried_move {
            // Check for collisions
            //TODO: Consider looking up Walls directly in the Map instead of indirectly via the Collidable query, to
            // save CPU (drawback would be that walls couldnt be non-collidable anymore, if thats ever needed)
            let new_x = player_pos.x + move_coordinates.0;
            let new_y = player_pos.y + move_coordinates.1;

            for collidable in collidables_query.iter() {
                if new_x == collidable.x && new_y == collidable.y {
                    return; // Collision detected
                }
            }

            player_pos.x = new_x;
            player_pos.y = new_y;

            // TODO: Right now I am lazy but this def. needs to
            // be an own function that translates coords to pixels
            // keeping in mind that bevy's pixel coords start from the middle of the screen
            player_tf.translation =
                Vec3::new(new_x as f32 * TILE_SIZE, new_y as f32 * TILE_SIZE, PLAYER_Z);
        }
    }
}
