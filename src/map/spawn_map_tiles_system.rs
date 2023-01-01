use crate::{
    components::position::Position, utils::render::map_pos_to_screen_pos, GameConfig, GameState,
};
use bevy::prelude::*;

use super::{game_map::GameMap, MaterialHandles, Tile, TileType};

/// Iterate over the game map and spawn a tile with the proper material for each cell of the map
pub fn spawn_map_tiles(
    mut commands: Commands,
    map: Res<GameMap>,
    materials: Res<MaterialHandles>,
    material_assets: Res<Assets<ColorMaterial>>,
    mut app_state: ResMut<State<GameState>>,
    game_config: Res<GameConfig>,
) {
    for (pos, tile) in map.tiles.iter() {
        let material: ColorMaterial;
        match tile {
            TileType::Floor => {
                material = material_assets
                    .get(materials.floor.clone())
                    .expect("missing floor material in ColorMaterial assets")
                    .clone()
            }
            TileType::Wall => {
                material = material_assets
                    .get(materials.wall.clone())
                    .expect("missing wall material in ColorMaterial assets")
                    .clone()
            }
        };

        let mut entity = commands.spawn();
        let scaled_tile_size =
            game_config.tile_properties.tile_size * game_config.tile_properties.tile_scale;
        entity
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: material.color,
                    custom_size: Some(Vec2::new(scaled_tile_size, scaled_tile_size)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: map_pos_to_screen_pos(
                        pos,
                        0.0,
                        game_config.tile_properties.tile_size,
                        &game_config.screen_dimensions,
                    ),
                    scale: Vec3::new(
                        game_config.tile_properties.tile_scale,
                        game_config.tile_properties.tile_scale,
                        0.0,
                    ),
                    ..Default::default()
                },
                visibility: Visibility { is_visible: false },
                ..Default::default()
            })
            .insert(Position { x: pos.x, y: pos.y })
            .insert(Tile {});
    }

    app_state
        .set(GameState::Render)
        .expect("failed to set game state after spawning tiles");
}
