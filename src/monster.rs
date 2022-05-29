use bevy::prelude::*;

use crate::{
    map::GameMap, player::Player, position::Position, viewshed::Viewshed, GameState, MONSTER_Z,
    SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE,
};

pub const MONSTER_FOV: i32 = 8;
pub const MONSTER_STARTING_HEALTH: i32 = 50;

pub struct MonsterPlugin {}

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::PlayerActive)
                .with_system(monster_ai.label("monster_movement").before("map_indexer")),
        );
    }
}

#[derive(Component)]
pub struct Monster {}

fn monster_ai(
    mut map: ResMut<GameMap>,
    mut monsters_and_player_set: ParamSet<(
        Query<(Entity, &mut Transform, &mut Position, &mut Viewshed), With<Monster>>,
        Query<&Position, With<Player>>,
    )>,
) {
    let player_pos = monsters_and_player_set
        .p1()
        .get_single()
        .expect("no player pos entity found")
        .clone();

    for (monster_entity, mut monster_tf, mut monster_pos, mut viewshed) in
        monsters_and_player_set.p0().iter_mut()
    {
        let mut sees_player = false;
        for viewshed_pos in &viewshed.visible_tiles {
            if player_pos == *viewshed_pos {
                sees_player = true;
                break;
            }
        }

        if sees_player {
            move_to_player(
                monster_entity,
                &mut monster_tf,
                &mut monster_pos,
                &player_pos,
                &mut map,
                &mut viewshed,
            );
        }
    }
}

fn move_to_player(
    monster_entity: Entity,
    monster_tf: &mut Transform,
    monster_pos: &mut Position,
    player_pos: &Position,
    map: &mut GameMap,
    viewshed: &mut Viewshed,
) {
    let position = monster_pos.clone();
    let path_result_opt = pathfinding::directed::astar::astar(
        &position,
        |position| map.get_traversable_neighbours_with_distance(position),
        |pos| pos.get_airline_distance(player_pos),
        |pos| pos.is_adjacent_to(player_pos),
    );

    if let Some(path_result) = path_result_opt {
        if path_result.0.len() > 1 {
            // unblock old position
            map.remove_blocked(&monster_pos);
            map.remove_tile_content(&monster_pos);

            monster_pos.x = path_result.0[1].x;
            monster_pos.y = path_result.0[1].y;

            // block new position
            map.set_blocked(monster_pos.clone());
            map.set_tile_content(monster_pos.clone(), monster_entity.clone());

            monster_tf.translation = Vec3::new(
                monster_pos.x as f32 * TILE_SIZE - SCREEN_WIDTH / 2.0,
                monster_pos.y as f32 * TILE_SIZE - SCREEN_HEIGHT / 2.0,
                MONSTER_Z,
            );

            viewshed.dirty = true; // Monster moved, re-compute viewshed
        }
    }
}
