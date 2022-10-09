use bevy::prelude::*;

use crate::components::{suffer_damage::DamageTracker, combatstats::CombatStats};
use crate::monster::MONSTER_TURN_LABEL;
use crate::player::{Player, PLAYER_TURN_LABEL};
use crate::user_interface::ActionLog;
use crate::GameState;
use crate::{map::GameMap, components::position::Position};

pub struct DamageSystemPlugin {}

const APPLY_DAMAGE_LABEL: &str = "apply_damage";

impl Plugin for DamageSystemPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system(apply_damage);
        app.add_system_set(
            SystemSet::on_update(GameState::PlayerTurn)
                .with_system(apply_damage)
                .after(PLAYER_TURN_LABEL)
                .label(APPLY_DAMAGE_LABEL),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::PlayerTurn)
                .with_system(collect_dead)
                .after(APPLY_DAMAGE_LABEL),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::MonsterTurn)
                .with_system(apply_damage)
                .after(MONSTER_TURN_LABEL),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::MonsterTurn)
                .with_system(collect_dead)
                .after(APPLY_DAMAGE_LABEL),
        );
    }
}

fn apply_damage(
    mut damage_tracker: ResMut<DamageTracker>,
    mut combat_query: Query<(Entity, &mut CombatStats)>,
) {
    for (entity, mut combat_stats) in combat_query.iter_mut() {
        if let Some(damage) = damage_tracker.0.get_mut(&entity) {
            combat_stats.hurt(damage.amount.iter().sum());
            damage.amount.clear();
        }
    }
}

fn collect_dead(
    mut commands: Commands,
    mut map: ResMut<GameMap>,
    player_entity: Query<Entity, With<Player>>,
    combat_stats_query: Query<(Entity, &Position, &CombatStats)>,
    mut action_log: ResMut<ActionLog>,
) {
    for (entity, position, combat_stats) in combat_stats_query.iter() {
        if combat_stats.hp <= 0 {
            let player = player_entity
                .get_single()
                .expect("Found 0 or more than one player in collect_dead");

            let text = if player.id() == entity.id() {
                "You died!"
            } else {
                "Monster died"
            };

            action_log.entries.push(text.to_owned());

            commands.entity(entity).despawn();

            map.remove_blocked(position);
            map.remove_tile_content(position);
        }
    }
}
