use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

use bevy::prelude::*;
use rand::{prelude::ThreadRng, Rng};

use crate::{
    components::position::Position,
    configs::game_settings::GameplaySettings,
    spawner::{self, spawn_player},
    utils::rectangle::Rectangle,
    GameConfig, GameState, MapProperties, ScreenDimensions, TileProperties,
};

use super::{game_map::GameMap, MainCamera, MaterialHandles, TileType};

/// Generate the map, load materials and spawn the camera.
/// Sets the game to `GameState::MapLoaded` when done
pub fn generate_map(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut app_state: ResMut<State<GameState>>,
    game_config: Res<GameConfig>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera {});

    let material_handles = MaterialHandles {
        player: materials.add(Color::rgb_u8(0, 163, 204).into()),
        wall: materials.add(Color::rgb_u8(217, 217, 217).into()),
        health_potion: materials.add(Color::rgb_u8(34, 139, 34).into()),
        wall_out_of_sight: materials.add(Color::rgb_u8(140, 140, 140).into()),
        monster: materials.add(Color::rgb_u8(204, 41, 0).into()),
        friendly: materials.add(Color::rgb_u8(51, 255, 178).into()),
        floor: materials.add(Color::rgb_u8(10, 10, 120).into()),
        floor_out_of_sight: materials.add(Color::rgb_u8(6, 6, 70).into()),
    };
    commands.insert_resource(material_handles.clone());

    let map = build_map(
        &mut commands,
        &game_config.map_properties,
        &game_config.tile_properties,
        &game_config.screen_dimensions,
        &game_config.gameplay_settings,
    );

    commands.insert_resource(map);

    app_state
        .set(GameState::MapLoaded)
        .expect("failed to set game state in map.setup()");
}

/// Generate the world map by randomly generating rooms
fn build_map(
    commands: &mut Commands,
    map_properties: &MapProperties,
    tile_properties: &TileProperties,
    screen_dimensions: &ScreenDimensions,
    gameplay_settings: &GameplaySettings,
) -> GameMap {
    let mut tiles: HashMap<Position, TileType> = HashMap::new();
    let mut collidables: HashSet<Position> = HashSet::new();

    // Init world to be all walls
    for x in 0..map_properties.map_width {
        for y in 0..map_properties.map_height {
            tiles.insert(Position { x, y }, TileType::Wall);
            collidables.insert(Position { x, y });
        }
    }

    let mut game_map = GameMap::new(
        map_properties.map_height,
        map_properties.map_width,
        tiles,
        HashSet::new(),
        collidables,
        HashMap::new(),
    );

    generate_rooms(
        commands,
        &mut game_map,
        tile_properties,
        screen_dimensions,
        gameplay_settings,
        map_properties.max_rooms,
    );

    game_map
}

/// Creates non-overlapping rooms on the map and fills them with the player (first room) or
/// monsters (all other rooms)
fn generate_rooms(
    commands: &mut Commands,
    game_map: &mut GameMap,
    tile_properties: &TileProperties,
    screen_dimensions: &ScreenDimensions,
    gameplay_settings: &GameplaySettings,
    max_rooms: u32,
) {
    let room_min_height = game_map.height / 10;
    let room_min_width = game_map.width / 10;
    let room_max_height = game_map.height / 5;
    let room_max_width = game_map.width / 5;
    let mut rooms: Vec<Rectangle> = vec![];

    let mut rand = rand::thread_rng();

    for room_no in 0..max_rooms {
        let new_room = generate_room(
            room_min_height,
            room_max_height,
            room_min_width,
            room_max_width,
            game_map.width,
            game_map.height,
            &mut rand,
        );

        for room in rooms.iter() {
            if room.intersects(&new_room) {
                continue;
            }
        }

        if room_no == 0 {
            // Place player in first room
            let (x, y) = &new_room.get_center();
            spawn_player(
                commands,
                Position { x: *x, y: *y },
                tile_properties,
                screen_dimensions,
                gameplay_settings,
            );
        } else {
            spawner::spawn_room(
                commands,
                &new_room,
                &mut rand,
                tile_properties,
                screen_dimensions,
                gameplay_settings,
            );
        }

        apply_room_to_map(game_map, &new_room);
        rooms.push(new_room);
    }

    let mut prev_room: Option<&Rectangle> = None;
    for room in rooms.iter() {
        if let Some(prev) = prev_room {
            let (prev_x, prev_y) = prev.get_center();
            let (curr_x, curr_y) = room.get_center();

            // Mix tunnel generation up a little
            let tunnel_horizontal: Rectangle;
            let tunnel_vertical: Rectangle;
            if rand.gen_range(1..=2) == 1 {
                tunnel_horizontal = generate_horizontal_tunnel(prev_x, curr_x, prev_y);
                tunnel_vertical = generate_vertical_tunnel(prev_y, curr_y, curr_x);
            } else {
                tunnel_vertical = generate_vertical_tunnel(prev_y, curr_y, prev_x);
                tunnel_horizontal = generate_horizontal_tunnel(prev_x, curr_x, curr_y);
            }
            apply_room_to_map(game_map, &tunnel_horizontal);
            apply_room_to_map(game_map, &tunnel_vertical);
        }
        prev_room = Some(room);
    }
}

fn generate_room(
    min_height: i32,
    max_height: i32,
    min_width: i32,
    max_width: i32,
    map_width: i32,
    map_height: i32,
    rand: &mut ThreadRng,
) -> Rectangle {
    let height = rand.gen_range(min_height..=max_height);
    let width = rand.gen_range(min_width..=max_width);
    let x = rand.gen_range(1..(map_width - width));
    let y = rand.gen_range(1..(map_height - height));

    Rectangle::new(x, y, width, height)
}

fn generate_horizontal_tunnel(x1: i32, x2: i32, y: i32) -> Rectangle {
    let left = min(x1, x2);
    let right = max(x1, x2);
    Rectangle {
        x1: left,
        x2: right,
        y1: y,
        y2: y,
    }
}

fn generate_vertical_tunnel(y1: i32, y2: i32, x: i32) -> Rectangle {
    let top = min(y1, y2);
    let bottom = max(y1, y2);
    Rectangle {
        x1: x,
        x2: x,
        y1: top,
        y2: bottom,
    }
}

/// Manifests a room in the game world
fn apply_room_to_map(map: &mut GameMap, room: &Rectangle) {
    for x in room.x1..=room.x2 {
        for y in room.y1..=room.y2 {
            let pos = Position { x, y };
            map.set_traversable(&pos);
            map.tiles.insert(pos, TileType::Floor);
        }
    }
}
