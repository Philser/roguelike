use bevy::prelude::*;

use crate::components::CombatStats::CombatStats;

pub struct DeadCollectorPlugin {}

impl Plugin for DeadCollectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collect_dead);
    }
}

// Think about implementing DamageSystem
fn collect_dead(mut commands: Commands, combat_stats_query: Query<(Entity, &CombatStats)>) {
    for (entity, combat_stats) in combat_stats_query.iter() {
        if combat_stats.hp <= 0 {
            bevy::log::info!("Despawning dead entity");
            commands.entity(entity).despawn();
        }
    }
}
