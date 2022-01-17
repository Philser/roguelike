use bevy::prelude::*;

use crate::{map::GameMap, position::Position, components::collidable::Collidable};

pub struct MapIndexerPlugin {}

impl Plugin for MapIndexerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(index_map.system().label("map_indexer"));
    }
}

fn index_map(collidables_query: Query<(&Collidable, &Position)>, mut map: ResMut<GameMap>) {
    map.populate_blocked();
    for (_, pos) in collidables_query.iter() {
        // println!("I found a collidable at: {:?}:{:?}", pos.x, pos.y);
        map.set_blocked(Position { x: pos.x, y: pos.y })
    }
}
