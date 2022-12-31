mod components;
mod configs;
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
use configs::game_settings::{
    GameConfig, GameplaySettings, MapProperties, ScreenDimensions, TileProperties, SCREEN_HEIGHT,
    SCREEN_WIDTH,
};
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
                item_z: 3.0,
                monster_z: 5.0,
                player_z: 5.0,
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
            gameplay_settings: GameplaySettings {
                player_starting_health: 100,
                health_potion_heal_amount: 20,
                monster_starting_health: 50,
            },
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
