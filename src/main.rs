mod damageable;
mod map;
mod monster;
mod player;
mod position;

use bevy::prelude::*;
use map::GameMapPlugin;
use player::PlayerPlugin;

const TILE_SIZE: f32 = 16.0;
const PLAYER_Z: f32 = 5.0;

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
            width: 1280.0,
            height: 720.0,
            title: "Roguelike".to_owned(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_state(GameState::LoadingResources)
        .add_plugin(GameMapPlugin {})
        .add_plugin(PlayerPlugin {})
        .run();
}
