use bevy::prelude::*;

use crate::{
    components::position::Position,
    components::{
        combat_stats::CombatStats, damage::DamageTracker, damage::SufferDamage, item::Confusion,
    },
    configs::game_settings::TileProperties,
    map::game_map::GameMap,
    player::Player,
    user_interface::ActionLog,
    utils::render::map_pos_to_screen_pos,
    viewshed::Viewshed,
    GameConfig, GameState, ScreenDimensions,
};

pub const MONSTER_FOV: i32 = 8;
pub const MONSTER_TURN_LABEL: &str = "monster_turn";

pub struct MonsterPlugin {}

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::MonsterTurn)
                .with_system(monster_ai.label(MONSTER_TURN_LABEL)),
        );
    }
}

#[derive(Component)]
pub struct Monster {}

fn monster_ai(
    mut commands: Commands,
    mut map: ResMut<GameMap>,
    mut damage_tracker: ResMut<DamageTracker>,
    mut app_state: ResMut<State<GameState>>,
    mut monsters_and_player_set: ParamSet<(
        Query<
            (
                Entity,
                &mut Transform,
                &mut Position,
                &mut Viewshed,
                &CombatStats,
                Option<&mut Confusion>,
            ),
            With<Monster>,
        >,
        Query<(Entity, &Position), With<Player>>,
    )>,
    mut action_log: ResMut<ActionLog>,
    game_config: Res<GameConfig>,
) {
    let q = monsters_and_player_set.p1();
    let player_tuple = q
        .get_single()
        .expect("failed to retrieve player entity query result");

    let player_pos = player_tuple.1.to_owned();
    let player_entity = player_tuple.0.to_owned();

    let action_log_ref = action_log.as_mut();

    for (
        monster_entity,
        mut monster_tf,
        mut monster_pos,
        mut viewshed,
        combat_stats,
        confusion_option,
    ) in monsters_and_player_set.p0().iter_mut()
    {
        if let Some(mut confusion) = confusion_option {
            if confusion.turns > 0 {
                // Monster is confused, does nothing
                confusion.turns -= 1;
                continue;
            } else {
                commands.entity(monster_entity).remove::<Confusion>();
            }
        }

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
                combat_stats,
                &mut map,
                &mut viewshed,
                &mut damage_tracker,
                player_entity,
                action_log_ref,
                &game_config.tile_properties,
                &game_config.screen_dimensions,
            );
        }
    }

    app_state
        .set(GameState::Render)
        .expect("failed to set game state in monster_ai");
}

fn move_to_player(
    monster_entity: Entity,
    monster_tf: &mut Transform,
    monster_pos: &mut Position,
    player_pos: &Position,
    monster_combat_stats: &CombatStats,
    map: &mut GameMap,
    viewshed: &mut Viewshed,
    damage_tracker: &mut ResMut<DamageTracker>,
    player_entity: Entity,
    action_log: &mut ActionLog,
    tile_properties: &TileProperties,
    screen_dimensions: &ScreenDimensions,
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
            map.remove_blocked(monster_pos);
            map.remove_tile_content(monster_pos);

            monster_pos.x = path_result.0[1].x;
            monster_pos.y = path_result.0[1].y;

            // block new position
            map.set_blocked(monster_pos.clone());
            map.set_tile_content(monster_pos.clone(), monster_entity);

            monster_tf.translation = map_pos_to_screen_pos(
                monster_pos,
                tile_properties.monster_z,
                tile_properties.tile_size,
                screen_dimensions,
            );

            viewshed.dirty = true; // Monster moved, re-compute viewshed
        } else {
            // attack the player in melee
            SufferDamage::add_damage(
                damage_tracker,
                player_entity,
                monster_combat_stats.power,
                action_log,
                false,
            );
            bevy::log::info!("Player has been hit with {}", monster_combat_stats.power,);
        }
    }
}
