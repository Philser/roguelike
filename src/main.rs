mod components;
mod config;
mod damage_system;
mod inventory;
mod map;
mod monster;
mod player;
mod spawner;
mod user_interface;
mod utils;
mod viewshed;

use std::collections::HashMap;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, winit::WinitSettings};
use components::{damage::DamageTracker, user_input::UserInput};
use damage_system::DamageSystemPlugin;
use inventory::plugin::InventorySystemPlugin;
use map::GameMapPlugin;
use monster::MonsterPlugin;
use player::PlayerPlugin;
use user_interface::UIPlugin;
use viewshed::ViewshedPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    LoadingResources,
    MapLoaded,
    Render,
    AwaitingActionInput,
    PlayerTurn,
    Targeting,
    MonsterTurn,

    SetupInventoryScreen,
    RenderInventory,
    AwaitingInventoryInput,
}

pub struct GameConfig {
    tile_properties: TileProperties,
    screen_dimensions: ScreenDimensions,
    map_properties: MapProperties,
    player_z: f32,
    monster_z: f32,
    item_z: f32,
}

pub const SCREEN_HEIGHT: f32 = 720.0;
pub const SCREEN_WIDTH: f32 = 1280.0;

pub struct ScreenDimensions {
    screen_height: f32,
    screen_width: f32,
}

pub struct TileProperties {
    tile_size: f32,
    tile_scale: f32,
}

pub struct MapProperties {
    map_height: i32,
    map_width: i32,
    max_rooms: u32,
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
        .insert_resource(GameConfig {
            tile_properties: TileProperties {
                tile_scale: 1.0,
                tile_size: 16.0,
            },
            screen_dimensions: ScreenDimensions {
                screen_height: SCREEN_HEIGHT,
                screen_width: SCREEN_WIDTH,
            },
            map_properties: MapProperties {
                map_height: 30,
                map_width: 60,
                max_rooms: 10,
            },
            item_z: 3.0,
            monster_z: 5.0,
            player_z: 5.0,
        })
        .insert_resource(DamageTracker(HashMap::new()))
        .insert_resource(UserInput { x: 0, y: 0 })
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_state(GameState::LoadingResources)
        .add_plugin(UIPlugin {})
        .add_plugin(GameMapPlugin {})
        .add_plugin(PlayerPlugin {})
        .add_plugin(ViewshedPlugin {})
        .add_plugin(MonsterPlugin {})
        .add_plugin(DamageSystemPlugin {})
        .add_plugin(InventorySystemPlugin {})
        .run();
}
