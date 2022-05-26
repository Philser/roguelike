use bevy::prelude::*;

use crate::{components::collidable::Collidable, map::GameMap, position::Position};

pub struct MapIndexerPlugin {}

impl Plugin for MapIndexerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(
            index_map
                .system()
                .label("map_indexer")
                .after("monster_movement")
                .after("player_movement"),
        );
    }
}

fn index_map(collidables_query: Query<(&Collidable, &Position)>, mut map: ResMut<GameMap>) {
    // map.populate_blocked();
    // for (_, pos) in collidables_query.iter() {
    //     map.set_blocked(Position { x: pos.x, y: pos.y })
    // }
}
