use std::collections::HashSet;

use bevy::prelude::*;

use crate::{components::position::Position, player::Player, viewshed::Viewshed, GameState};

use super::{game_map::GameMap, MaterialHandles, Tile, TileType};

/// Render everything that is visible to the player in the world, i.e. tiles, monsters, and the player
pub fn render_map(
    map: Res<GameMap>,
    materials: Res<MaterialHandles>,
    material_assets: Res<Assets<ColorMaterial>>,
    mut viewshed_query: Query<&mut Viewshed, With<Player>>,
    tile_query: Query<(&mut Visibility, &mut Sprite, &Position, With<Tile>)>,
    mut monster_and_items: Query<(&mut Visibility, &Position, Without<Tile>)>,
    mut app_state: ResMut<State<GameState>>,
) {
    let mut visibles: HashSet<Position> = HashSet::new();
    let mut player_viewshed = viewshed_query
        .get_single_mut()
        .expect("Expected player viewshed");

    if player_viewshed.dirty {
        player_viewshed.dirty = false;

        for pos in &player_viewshed.visible_tiles {
            visibles.insert(Position { x: pos.x, y: pos.y });
        }

        render_tiles(&map, &materials, material_assets, tile_query, &visibles);

        // Render monsters, items and player
        for (mut visible_entity, entity_pos, _) in monster_and_items.iter_mut() {
            if visibles.contains(entity_pos) {
                // Render everything that is currently visible for the player in its original color
                visible_entity.is_visible = true;
            } else {
                visible_entity.is_visible = false;
            }
        }
    }

    app_state
        .set(GameState::AwaitingActionInput)
        .expect("failed to set game state in render_map");
}

/// Part of the `render_map` system. Renders tiles of the game map.
fn render_tiles(
    map: &Res<GameMap>,
    materials: &Res<MaterialHandles>,
    material_assets: Res<Assets<ColorMaterial>>,
    mut tile_query: Query<(&mut Visibility, &mut Sprite, &Position, With<Tile>)>,
    visibles: &HashSet<Position>,
) {
    for (mut visible_entity, mut sprite, entity_pos, _) in tile_query.iter_mut() {
        let tile_type = map.tiles.get(entity_pos).unwrap();
        let material_handler: Handle<ColorMaterial>;

        if visibles.contains(entity_pos) {
            // Render everything that is currently visible for the player in its original color
            visible_entity.is_visible = true;
            match *tile_type {
                TileType::Floor => material_handler = materials.floor.clone(),
                TileType::Wall => material_handler = materials.wall.clone(),
            }
        } else if map.visited_tiles.contains(entity_pos) {
            // Render the visited, currently out of sight parts of the map (tiles) in a different color
            visible_entity.is_visible = true;
            match *tile_type {
                TileType::Floor => material_handler = materials.floor_out_of_sight.clone(),
                TileType::Wall => material_handler = materials.wall_out_of_sight.clone(),
            }
        } else {
            visible_entity.is_visible = false;
            continue;
        }

        let color = material_assets
            .get(material_handler)
            .expect("missing asset for floor tile")
            .color;
        sprite.color = color;
    }
}
