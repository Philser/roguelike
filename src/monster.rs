use bevy::prelude::*;

use crate::{map::GameMap, player::Player, position::Position, viewshed::Viewshed, GameState};

pub const MONSTER_FOV: i32 = 8;
pub const MONSTER_STARTING_HEALTH: i32 = 50;

pub struct MonsterPlugin {}

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system_set(
            SystemSet::on_in_stack_update(GameState::PlayerActive).with_system(monster_ai.system()),
        );
    }
}

pub struct Monster {}

fn monster_ai(
    map: Res<GameMap>,
    mut monster_query: Query<(&mut Position, &mut Viewshed), With<Monster>>,
    player_query: Query<&Position, With<Player>>,
) {
    let player_pos = player_query.single().expect("Error querying the player");

    for (mut pos, mut viewshed) in monster_query.iter_mut() {
        let mut sees_player = false;
        for viewshed_pos in &viewshed.visible_tiles {
            if *player_pos == *viewshed_pos {
                sees_player = true;
                break;
            }
        }

        if sees_player {
            move_to_player(&mut pos, player_pos, &map, &mut viewshed);
        }
    }
}

fn move_to_player(
    monster_pos: &mut Position,
    player_pos: &Position,
    map: &GameMap,
    viewshed: &mut Viewshed,
) {
    let position = monster_pos.clone();
    let path_result = pathfinding::directed::astar::astar(
        &position,
        |position| map.get_traversable_neighbours_with_distance(&position),
        |pos| pos.get_airline_distance(&player_pos), // Every neighbour is one step away
        |pos| pos == player_pos,
    )
    .expect("Expected path from A*");

    monster_pos.x = path_result.0[0].x;
    monster_pos.y = path_result.0[0].y;

    viewshed.dirty = true; // Monster moved, re-compute viewshed
}
