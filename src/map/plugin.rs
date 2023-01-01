use bevy::prelude::{App, Plugin, SystemSet};

use crate::GameState;

use super::generate_map_system::generate_map;
use super::render_map_system::render_map;
use super::spawn_map_tiles_system::spawn_map_tiles;

pub struct GameMapPlugin {}

impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(generate_map)
            .add_system_set(SystemSet::on_enter(GameState::MapLoaded).with_system(spawn_map_tiles))
            .add_system_set(
                SystemSet::on_update(GameState::Render)
                    .with_system(render_map)
                    .label("render_map"),
            );
    }
}
