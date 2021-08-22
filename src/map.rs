use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    fs::File,
};

use bevy::prelude::*;
use rand::{prelude::ThreadRng, Rng};

use crate::{
    damageable::Damageable,
    monster::Monster,
    player::{Player, PLAYER_FOV, PLAYER_STARTING_HEALTH},
    position::Position,
    utils::rectangle::Rectangle,
    viewshed::Viewshed,
    Collidable, GameState, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE,
};

const SCALE: f32 = 1.0;
const MAP_HEIGHT: i32 = 30;
const MAP_WIDTH: i32 = 60;
const MAX_ROOMS: i32 = 10;

pub struct GameMapPlugin {}

impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system_set(
                SystemSet::on_enter(GameState::MapLoaded).with_system(spawn_map_tiles.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MapLoaded).with_system(render_map.system()),
            );
    }
}

pub struct GameMap {
    pub height: i32,
    pub width: i32,
    pub tiles: HashMap<MapPosition, TileType>,
    pub visited_tiles: HashSet<MapPosition>,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct MapPosition {
    pub x: i32,
    pub y: i32,
}

pub struct Materials {
    pub player: Handle<ColorMaterial>,
    pub wall: Handle<ColorMaterial>,
    pub wall_out_of_sight: Handle<ColorMaterial>,
    pub monster: Handle<ColorMaterial>,
    pub friendly: Handle<ColorMaterial>,
    pub floor: Handle<ColorMaterial>,
    pub floor_out_of_sight: Handle<ColorMaterial>,
}

#[derive(PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
}
#[derive(Debug, serde::Deserialize)]
pub struct Level {
    layout: Vec<String>,
}

/// Manifests a room in the game world
fn apply_room_to_map(map: &mut GameMap, room: &Rectangle) {
    for x in room.x1..=room.x2 {
        for y in room.y1..=room.y2 {
            map.tiles.insert(MapPosition { x, y }, TileType::Floor);
        }
    }
}

/// Generate the world map by randomly generating rooms
fn generate_map(mut commands: &mut Commands, materials: &Materials) -> GameMap {
    let mut tiles: HashMap<MapPosition, TileType> = HashMap::new();

    // Init world to be all walls
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            tiles.insert(MapPosition { x, y }, TileType::Wall);
        }
    }

    let mut game_map = GameMap {
        height: MAP_HEIGHT,
        width: MAP_WIDTH,
        tiles,
        visited_tiles: HashSet::new(),
    };

    let room_min_height = MAP_HEIGHT / 10;
    let room_min_width = MAP_WIDTH / 10;
    let room_max_height = MAP_HEIGHT / 5;
    let room_max_width = MAP_WIDTH / 5;
    let mut rooms: Vec<Rectangle> = vec![];

    let mut rand = rand::thread_rng();

    for room_no in 0..MAX_ROOMS {
        let new_room = generate_room(
            room_min_height,
            room_max_height,
            room_min_width,
            room_max_width,
            &mut rand,
        );

        if room_no == 0 {
            // Place player in first room
            let (x, y) = &new_room.get_center();
            spawn_player(&mut commands, &materials, *x, *y);
        }

        let mut room_ok = true;
        for room in rooms.iter() {
            if room.intersects(&new_room) {
                // Drop room
                room_ok = false;
            }
        }

        if room_ok {
            apply_room_to_map(&mut game_map, &new_room);
            rooms.push(new_room);
        }
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
            apply_room_to_map(&mut game_map, &tunnel_horizontal);
            apply_room_to_map(&mut game_map, &tunnel_vertical);
        }
        prev_room = Some(room);
    }

    game_map
}

fn spawn_player(commands: &mut Commands, materials: &Materials, x: i32, y: i32) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                size: Vec2::new(TILE_SIZE * SCALE, TILE_SIZE * SCALE),
                ..Default::default()
            },
            material: materials.player.clone(),
            transform: Transform {
                translation: Vec3::new(
                    x as f32 * TILE_SIZE - SCREEN_WIDTH / 2.0,
                    y as f32 * TILE_SIZE - SCREEN_HEIGHT / 2.0,
                    0.0,
                ),
                scale: Vec3::new(SCALE, SCALE, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position {
            x: x as i32,
            y: y as i32,
        })
        .insert(Damageable {
            health: PLAYER_STARTING_HEALTH,
        })
        .insert(Viewshed {
            visible_tiles: vec![],
            range: PLAYER_FOV,
        })
        .insert(Player {});
}

fn generate_room(
    min_height: i32,
    max_height: i32,
    min_width: i32,
    max_width: i32,
    rand: &mut ThreadRng,
) -> Rectangle {
    let height = rand.gen_range(min_height..=max_height);
    let width = rand.gen_range(min_width..=max_width);
    let x = rand.gen_range(1..(MAP_WIDTH - width));
    let y = rand.gen_range(1..(MAP_HEIGHT - height));

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

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut app_state: ResMut<State<GameState>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let materials = Materials {
        player: materials.add(Color::rgb_u8(0, 163, 204).into()),
        wall: materials.add(Color::rgb_u8(217, 217, 217).into()),
        wall_out_of_sight: materials.add(Color::rgb_u8(140, 140, 140).into()),
        monster: materials.add(Color::rgb_u8(204, 41, 0).into()),
        friendly: materials.add(Color::rgb_u8(51, 255, 178).into()),
        floor: materials.add(Color::rgb_u8(10, 10, 120).into()),
        floor_out_of_sight: materials.add(Color::rgb_u8(6, 6, 70).into()),
    };

    let map = generate_map(&mut commands, &materials);

    commands.insert_resource(map);

    commands.insert_resource(materials);
    app_state.set(GameState::MapLoaded).unwrap();
}

fn render_map(
    map: Res<GameMap>,
    materials: Res<Materials>,
    viewshed_query: Query<(&Viewshed, &Player)>,
    mut tile_query: Query<(
        &mut Visible,
        &mut Handle<ColorMaterial>,
        &MapPosition,
        Without<Player>,
    )>,
) {
    let mut visibles: HashSet<MapPosition> = HashSet::new();
    if let Ok((viewshed, _)) = viewshed_query.single() {
        for pos in &viewshed.visible_tiles {
            visibles.insert(MapPosition { x: pos.x, y: pos.y });
        }
    }

    for (mut visible_entity, mut handle, entity_pos, _) in tile_query.iter_mut() {
        let tile_type = map.tiles.get(entity_pos).unwrap();
        if visibles.contains(entity_pos) {
            visible_entity.is_visible = true;
            match tile_type {
                &TileType::Floor => {
                    handle.id = materials.floor.clone().id;
                }
                &TileType::Wall => {
                    handle.id = materials.wall.clone().id;
                }
            }
        } else if map.visited_tiles.contains(entity_pos) {
            match tile_type {
                &TileType::Floor => {
                    handle.id = materials.floor_out_of_sight.clone().id;
                }
                &TileType::Wall => {
                    handle.id = materials.wall_out_of_sight.clone().id;
                }
            }
        } else {
            visible_entity.is_visible = false;
        }
    }
}

fn spawn_map_tiles(mut commands: Commands, map: Res<GameMap>, materials: Res<Materials>) {
    for (pos, tile) in map.tiles.iter() {
        let material: Handle<ColorMaterial>;
        match tile {
            TileType::Floor => material = materials.floor.clone(),
            TileType::Wall => material = materials.wall.clone(),
        };

        let mut entity = commands.spawn();
        entity
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    size: Vec2::new(TILE_SIZE * SCALE, TILE_SIZE * SCALE),
                    ..Default::default()
                },
                material: material.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        pos.x as f32 * TILE_SIZE - SCREEN_WIDTH / 2.0,
                        pos.y as f32 * TILE_SIZE - SCREEN_HEIGHT / 2.0,
                        0.0,
                    ),
                    scale: Vec3::new(SCALE, SCALE, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(MapPosition { x: pos.x, y: pos.y });

        if *tile == TileType::Wall {
            entity.insert(Collidable { x: pos.x, y: pos.y });
        }
    }
}
