use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

use bevy::prelude::*;
use rand::{prelude::ThreadRng, Rng};

use crate::{
    damageable::Damageable,
    monster::{Monster, MONSTER_FOV, MONSTER_STARTING_HEALTH},
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

/// A structure representing the game world as a collection of points.
/// The upper left corner is at `Position` (0, 0), the lower right corner
/// is at (width - 1, height - 1).
pub struct GameMap {
    /// Height in
    pub height: i32,
    pub width: i32,
    pub tiles: HashMap<Position, TileType>,
    pub visited_tiles: HashSet<Position>,
}

impl GameMap {
    pub fn new(
        height: i32,
        width: i32,
        tiles: HashMap<Position, TileType>,
        visited_tiles: HashSet<Position>,
    ) -> Self {
        return GameMap {
            height,
            width,
            tiles,
            visited_tiles,
        };
    }

    /// Determines whether a given point in the map is an exit (not a wall).
    pub fn is_exit(&self, position: Position) -> bool {
        if position.x < 0 || position.x >= self.width || position.y < 0 || position.y >= self.height
        {
            return false;
        }

        return self.tiles.get(&position).unwrap() == &TileType::Floor;
    }
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

pub struct Tile {}

/// Manifests a room in the game world
fn apply_room_to_map(map: &mut GameMap, room: &Rectangle) {
    for x in room.x1..=room.x2 {
        for y in room.y1..=room.y2 {
            map.tiles.insert(Position { x, y }, TileType::Floor);
        }
    }
}

/// Generate the world map by randomly generating rooms
fn generate_map(mut commands: &mut Commands, materials: &Materials) -> GameMap {
    let mut tiles: HashMap<Position, TileType> = HashMap::new();

    // Init world to be all walls
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            tiles.insert(Position { x, y }, TileType::Wall);
        }
    }

    let mut game_map = GameMap::new(MAP_HEIGHT, MAP_WIDTH, tiles, HashSet::new());

    generate_rooms(&mut commands, &materials, &mut game_map);

    game_map
}

/// Creates non-overlapping rooms on the map and fills them with the player (first room) or
/// monsters (all other rooms)
fn generate_rooms(mut commands: &mut Commands, materials: &Materials, mut game_map: &mut GameMap) {
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

        let mut room_ok = true;
        for room in rooms.iter() {
            if room.intersects(&new_room) {
                // Drop room
                room_ok = false;
            }
        }

        if room_ok {
            let (x, y) = &new_room.get_center();
            if room_no == 0 {
                // Place player in first room
                spawn_player(&mut commands, &materials, Position { x: *x, y: *y });
            } else {
                // Spawn monster in all other rooms
                spawn_monster(&mut commands, &materials, Position { x: *x, y: *y });
            }

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
}

fn spawn_player(commands: &mut Commands, materials: &Materials, pos: Position) {
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
                    pos.x as f32 * TILE_SIZE - SCREEN_WIDTH / 2.0,
                    pos.y as f32 * TILE_SIZE - SCREEN_HEIGHT / 2.0,
                    5.0,
                ),
                scale: Vec3::new(SCALE, SCALE, 5.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(pos)
        .insert(Damageable {
            health: PLAYER_STARTING_HEALTH,
        })
        .insert(Viewshed {
            visible_tiles: vec![],
            range: PLAYER_FOV,
            dirty: true,
        })
        .insert(Player {});
}

fn spawn_monster(commands: &mut Commands, materials: &Materials, pos: Position) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                size: Vec2::new(TILE_SIZE * SCALE, TILE_SIZE * SCALE),
                ..Default::default()
            },
            material: materials.monster.clone(),
            transform: Transform {
                translation: Vec3::new(
                    pos.x as f32 * TILE_SIZE - SCREEN_WIDTH / 2.0,
                    pos.y as f32 * TILE_SIZE - SCREEN_HEIGHT / 2.0,
                    3.0,
                ),
                scale: Vec3::new(SCALE, SCALE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position {
            x: pos.x as i32,
            y: pos.y as i32,
        })
        .insert(Damageable {
            health: MONSTER_STARTING_HEALTH,
        })
        .insert(Viewshed {
            visible_tiles: vec![],
            range: MONSTER_FOV,
            dirty: false,
        })
        .insert(Collidable { x: pos.x, y: pos.y })
        .insert(Monster {});
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

/// Generate the map, load materials and spawn the camera.
/// Sets the game to `GameState::MapLoaded` when done
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

/// Render everything that is visible to the player in the world, i.e. tiles, monsters, and the player
fn render_map(
    map: Res<GameMap>,
    materials: Res<Materials>,
    mut viewshed_query: Query<(&mut Viewshed, &Player)>,
    tile_query: Query<(
        &mut Visible,
        &mut Handle<ColorMaterial>,
        &Position,
        With<Tile>,
    )>,
    mut monster_query: Query<(&mut Visible, &Position, Without<Tile>)>,
) {
    let mut visibles: HashSet<Position> = HashSet::new();
    if let Ok((mut viewshed, _)) = viewshed_query.single_mut() {
        if !viewshed.dirty {
            return; // Nothing to render, player didn't move
        }
        viewshed.dirty = false;

        for pos in &viewshed.visible_tiles {
            visibles.insert(Position { x: pos.x, y: pos.y });
        }
    }

    render_tiles(&map, &materials, tile_query, &visibles);

    // Render monsters and players
    for (mut visible_entity, entity_pos, _) in monster_query.iter_mut() {
        if visibles.contains(entity_pos) {
            // Render everything that is currently visible for the player in its original color
            visible_entity.is_visible = true;
        } else {
            visible_entity.is_visible = false;
        }
    }
}

/// Part of the `render_map` system. Renders tiles of the game map.
fn render_tiles(
    map: &Res<GameMap>,
    materials: &Res<Materials>,
    mut tile_query: Query<(
        &mut Visible,
        &mut Handle<ColorMaterial>,
        &Position,
        With<Tile>,
    )>,
    visibles: &HashSet<Position>,
) {
    for (mut visible_entity, mut handle, entity_pos, _) in tile_query.iter_mut() {
        let tile_type = map.tiles.get(entity_pos).unwrap();
        if visibles.contains(entity_pos) {
            // Render everything that is currently visible for the player in its original color
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
            // Render the visited, currently out of sight parts of the map (tiles) in a different color
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

/// Iterate over the game map and spawn a tile with the proper material for each cell of the map
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
            .insert(Position { x: pos.x, y: pos.y })
            .insert(Tile {});

        if *tile == TileType::Wall {
            entity.insert(Collidable { x: pos.x, y: pos.y });
        }
    }
}
