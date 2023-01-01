use bevy::ecs::component::Component;
use bevy::prelude::{Plugin, Query, Res, ResMut, With};
use doryen_fov::{FovAlgorithm, FovRecursiveShadowCasting, MapData};

use crate::{
    components::position::Position,
    map::{game_map::GameMap, TileType},
    monster::Monster,
    player::Player,
};

pub struct ViewshedPlugin;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Position>,
    pub range: i32,
    pub dirty: bool,
}

impl Plugin for ViewshedPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(populate_viewshed_player);
        app.add_system(populate_viewshed_monsters);
    }
}

fn populate_viewshed_player(
    mut map: ResMut<GameMap>,
    mut viewshed_player: Query<(&Position, &mut Viewshed, With<Player>)>,
) {
    let mut fov = FovRecursiveShadowCasting::new();

    let (entity_pos, mut viewshed, _) = viewshed_player.single_mut();
    let mut temp_map = MapData::new(map.width as usize, map.height as usize);

    // Find all walls within this area in the actual game world
    for x in 0..temp_map.width {
        for y in 0..temp_map.height {
            if let Some(tile) = map.tiles.get(&Position {
                x: x as i32,
                y: y as i32,
            }) {
                if *tile == TileType::Wall {
                    temp_map.set_transparent(x, y, false);
                }
            }
        }
    }

    temp_map.clear_fov();
    fov.compute_fov(
        &mut temp_map,
        entity_pos.x as usize, // Entity is always in the middle of the map
        entity_pos.y as usize, // Entity is always in the middle of the map
        viewshed.range as usize,
        true,
    );

    // Now find all the tiles that are visible and translate to real game map
    viewshed.visible_tiles.clear();
    for x in 0..temp_map.width {
        for y in 0..temp_map.height {
            if temp_map.is_in_fov(x, y) {
                let pos = Position {
                    x: x as i32,
                    y: y as i32,
                };
                viewshed.visible_tiles.push(pos.clone());
                map.visited_tiles.insert(pos);
            }
        }
    }
}

fn populate_viewshed_monsters(
    map: ResMut<GameMap>,
    mut viewshed_monsters: Query<(&Position, &mut Viewshed, With<Monster>)>,
) {
    let mut fov = FovRecursiveShadowCasting::new();

    for (entity_pos, mut viewshed, _) in viewshed_monsters.iter_mut() {
        let mut temp_map = MapData::new(map.width as usize, map.height as usize);

        // Find all walls within this area in the actual game world
        for x in 0..temp_map.width {
            for y in 0..temp_map.height {
                if let Some(tile) = map.tiles.get(&Position {
                    x: x as i32,
                    y: y as i32,
                }) {
                    if *tile == TileType::Wall {
                        temp_map.set_transparent(x, y, false);
                    }
                }
            }
        }

        temp_map.clear_fov();
        fov.compute_fov(
            &mut temp_map,
            entity_pos.x as usize, // Entity is always in the middle of the map
            entity_pos.y as usize, // Entity is always in the middle of the map
            viewshed.range as usize,
            true,
        );

        // Now find all the tiles that are visible and translate to real game map
        viewshed.visible_tiles.clear();
        for x in 0..temp_map.width {
            for y in 0..temp_map.height {
                if temp_map.is_in_fov(x, y) {
                    let pos = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    viewshed.visible_tiles.push(pos.clone());
                }
            }
        }
    }
}

// TODO: Revisit and figure out why this didnt work because I really think this could save computing power
fn _populate_viewshed_weird(map: Res<GameMap>, mut query: Query<(&Position, &mut Viewshed)>) {
    let mut fov = FovRecursiveShadowCasting::new();

    for (entity_pos, mut viewshed) in query.iter_mut() {
        // We only care about the part of the world that is within fov range anyway
        let mut temp_map = MapData::new(viewshed.range as usize * 2, viewshed.range as usize * 2);

        // Find all walls within this area in the actual game world
        for x in 0..temp_map.width {
            for y in 0..temp_map.height {
                if let Some(tile) = map.tiles.get(&Position {
                    x: x as i32 + entity_pos.x,
                    y: y as i32 + entity_pos.y,
                }) {
                    if *tile == TileType::Wall {
                        temp_map.set_transparent(x, y, false);
                    }
                }
            }
        }

        temp_map.clear_fov();
        fov.compute_fov(
            &mut temp_map,
            viewshed.range as usize, // Entity is always in the middle of the map
            viewshed.range as usize, // Entity is always in the middle of the map
            viewshed.range as usize,
            false,
        );

        // Now find all the tiles that are visible and translate to real game map
        viewshed.visible_tiles.clear();
        for x in 0..temp_map.width {
            for y in 0..temp_map.height {
                if temp_map.is_in_fov(x, y) {
                    viewshed.visible_tiles.push(Position {
                        x: x as i32 + entity_pos.x,
                        y: y as i32 + entity_pos.y,
                    })
                }
            }
        }
    }
}
