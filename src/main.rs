mod damageable;
mod map;
mod monster;
mod player;
mod position;
mod utils;
mod viewshed;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use map::GameMapPlugin;
use player::PlayerPlugin;
use viewshed::ViewshedPlugin;

const TILE_SIZE: f32 = 16.0;
const PLAYER_Z: f32 = 5.0;
const SCREEN_HEIGHT: f32 = 720.0;
const SCREEN_WIDTH: f32 = 1280.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    LoadingResources,
    MapLoaded,
}

struct Collidable {
    pub x: i32,
    pub y: i32,
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            title: "Roguelike".to_owned(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_state(GameState::LoadingResources)
        .add_plugin(GameMapPlugin {})
        .add_plugin(PlayerPlugin {})
        .add_plugin(ViewshedPlugin {})
        .run();
}
