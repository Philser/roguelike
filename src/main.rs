mod components;
mod damage_system;
mod map;
mod map_indexer;
mod monster;
mod player;
mod position;
mod utils;
mod viewshed;

use std::collections::HashMap;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use components::suffer_damage::DamageTracker;
use damage_system::DamageSystemPlugin;
use map::GameMapPlugin;
use map_indexer::MapIndexerPlugin;
use monster::MonsterPlugin;
use player::PlayerPlugin;
use viewshed::ViewshedPlugin;

const TILE_SIZE: f32 = 16.0;
const PLAYER_Z: f32 = 5.0;
const MONSTER_Z: f32 = 5.0;
const SCREEN_HEIGHT: f32 = 720.0;
const SCREEN_WIDTH: f32 = 1280.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    LoadingResources,
    MapLoaded,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            title: "Roguelike".to_owned(),
            ..Default::default()
        })
        .insert_resource(DamageTracker(HashMap::new()))
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_state(GameState::LoadingResources)
        .add_plugin(GameMapPlugin {})
        .add_plugin(PlayerPlugin {})
        .add_plugin(ViewshedPlugin {})
        .add_plugin(MonsterPlugin {})
        .add_plugin(MapIndexerPlugin {})
        .add_plugin(DamageSystemPlugin {})
        .run();
}
