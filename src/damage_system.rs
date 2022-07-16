use bevy::prelude::*;

use crate::components::{suffer_damage::DamageTracker, CombatStats::CombatStats};
use crate::monster::MONSTER_TURN_LABEL;
use crate::player::PLAYER_TURN_LABEL;
use crate::GameState;
use crate::{map::GameMap, position::Position};

pub struct DamageSystemPlugin {}

impl Plugin for DamageSystemPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system(apply_damage);
        app.add_system_set(
            SystemSet::on_update(GameState::PlayerTurn)
                .with_system(apply_damage)
                .after(PLAYER_TURN_LABEL),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::MonsterTurn)
                .with_system(apply_damage)
                .after(MONSTER_TURN_LABEL),
        );
        app.add_system(collect_dead);
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
    combat_stats_query: Query<(Entity, &Position, &CombatStats)>,
) {
    for (entity, position, combat_stats) in combat_stats_query.iter() {
        if combat_stats.hp <= 0 {
            bevy::log::info!("Despawning dead entity");
            commands.entity(entity).despawn();

            map.remove_blocked(position);
            map.remove_tile_content(position);
        }
    }
}
