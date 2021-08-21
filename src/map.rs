use std::{collections::HashMap, fs::File};

use bevy::prelude::*;
use rand::Rng;

use crate::{
    damageable::Damageable,
    monster::Monster,
    player::{Player, PLAYER_STARTING_HEALTH},
    position::Position,
    utils::rectangle::Rectangle,
    Collidable, GameState, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE,
};

const SCALE: f32 = 1.0;
const MAP_HEIGHT: i32 = 30;
const MAP_WIDTH: i32 = 60;
const MAX_ROOMS: i32 = 5;

pub struct GameMapPlugin {}

impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system()).add_system_set(
            SystemSet::on_enter(GameState::MapLoaded).with_system(render_map.system()),
        );
    }
}

pub struct GameMap {
    height: i32,
    width: i32,
    tiles: HashMap<MapPosition, TileType>,
}

#[derive(PartialEq, Eq, Hash)]
struct MapPosition {
    x: i32,
    y: i32,
}

pub struct Materials {
    pub player: Handle<ColorMaterial>,
    pub wall: Handle<ColorMaterial>,
    pub monster: Handle<ColorMaterial>,
    pub friendly: Handle<ColorMaterial>,
    pub floor: Handle<ColorMaterial>,
}

#[derive(PartialEq, Eq)]
enum TileType {
    Wall,
    Floor,
}
#[derive(Debug, serde::Deserialize)]
pub struct Level {
    layout: Vec<String>,
}

/// Parse level to create game map and entities like the player and enemies.
fn parse_level(commands: &mut Commands, materials: &Materials, level: Level) -> GameMap {
    let mut tiles: HashMap<MapPosition, TileType> = HashMap::new();
    let mut height = 0;
    let mut width = 0;

    for (y, row) in level.layout.iter().rev().enumerate() {
        // Without rev(), for some reason everything is upside-down
        height += 1;
        for (x, col) in row.chars().enumerate() {
            match col {
                '#' => {
                    tiles.insert(
                        MapPosition {
                            x: x as i32,
                            y: y as i32,
                        },
                        TileType::Wall,
                    );
                }
                '.' => {
                    tiles.insert(
                        MapPosition {
                            x: x as i32,
                            y: y as i32,
                        },
                        TileType::Floor,
                    );
                }
                '@' => {
                    // Add floor tile and render player on top of it
                    tiles.insert(
                        MapPosition {
                            x: x as i32,
                            y: y as i32,
                        },
                        TileType::Floor,
                    );

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
                                    x as f32 * TILE_SIZE, // TODO: Right now I am lazy but this def. needs to
                                    y as f32 * TILE_SIZE, // TODO: be an own function that takes half the window size instead of 500
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
                        .insert(Player {});
                }
                'm' => {
                    // Add floor tile and render monster on top of it
                    tiles.insert(
                        MapPosition {
                            x: x as i32,
                            y: y as i32,
                        },
                        TileType::Floor,
                    );

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
                                    x as f32 * TILE_SIZE, // TODO: Right now I am lazy but this def. needs to
                                    y as f32 * TILE_SIZE, // TODO: be an own function that takes half the window size instead of 500
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
                        .insert(Collidable {
                            x: x as i32,
                            y: y as i32,
                        })
                        .insert(Monster {});
                }
                unknown => panic!("Couldn't parse map due to unknown character: {}", unknown),
            }
            width += 1;
        }
    }

    GameMap {
        height,
        width,
        tiles,
    }
}

fn apply_room_to_map(map: &mut GameMap, room: &Rectangle) {
    for x in room.x1..=room.x2 {
        for y in room.y1..=room.y2 {
            map.tiles.insert(MapPosition { x, y }, TileType::Floor);
        }
    }
}

fn generate_map(mut commands: &mut Commands, materials: &Materials) -> GameMap {
    let mut tiles: HashMap<MapPosition, TileType> = HashMap::new();

    // Init world to be all walls

    println!("Init world");
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            tiles.insert(MapPosition { x, y }, TileType::Wall);
        }
    }

    let mut game_map = GameMap {
        height: MAP_HEIGHT,
        width: MAP_WIDTH,
        tiles,
    };

    let room_min_height = MAP_HEIGHT / 10;
    let room_min_width = MAP_WIDTH / 10;
    let room_max_height = MAP_HEIGHT / 5;
    let room_max_width = MAP_WIDTH / 5;
    let mut rooms: Vec<Rectangle> = vec![];

    for room_no in 0..MAX_ROOMS {
        let new_room = generate_room(
            room_min_height,
            room_max_height,
            room_min_width,
            room_max_width,
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
                    x as f32 * TILE_SIZE - SCREEN_WIDTH / 2.0, // TODO: Right now I am lazy but this def. needs to
                    y as f32 * TILE_SIZE - SCREEN_HEIGHT / 2.0, // TODO: be an own function that takes half the window size instead of 500
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
        .insert(Player {});
}

fn generate_room(min_height: i32, max_height: i32, min_width: i32, max_width: i32) -> Rectangle {
    let mut rand = rand::thread_rng();
    let height = rand.gen_range(min_height..=max_height);
    let width = rand.gen_range(min_width..=max_width);
    let x = rand.gen_range(0..=(MAP_WIDTH - width));
    let y = rand.gen_range(0..=(MAP_HEIGHT - height));

    Rectangle::new(x, y, width, height)
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
        monster: materials.add(Color::rgb_u8(204, 41, 0).into()),
        friendly: materials.add(Color::rgb_u8(51, 255, 178).into()),
        floor: materials.add(Color::rgb(0.01, 0.01, 0.12).into()),
    };

    let map = generate_map(&mut commands, &materials);

    commands.insert_resource(map);

    commands.insert_resource(materials);
    app_state.set(GameState::MapLoaded).unwrap();
}

fn render_map(mut commands: Commands, map: Res<GameMap>, materials: Res<Materials>) {
    println!("Render");
    for (pos, tile) in map.tiles.iter() {
        let material: Handle<ColorMaterial>;
        match tile {
            TileType::Floor => material = materials.floor.clone(),
            TileType::Wall => material = materials.wall.clone(),
        };

        let mut entity = commands.spawn();
        entity.insert_bundle(SpriteBundle {
            sprite: Sprite {
                size: Vec2::new(TILE_SIZE * SCALE, TILE_SIZE * SCALE),
                ..Default::default()
            },
            material: material.clone(),
            transform: Transform {
                translation: Vec3::new(
                    pos.x as f32 * TILE_SIZE - SCREEN_WIDTH / 2.0, // TODO: Right now I am lazy but this def. needs to
                    pos.y as f32 * TILE_SIZE - SCREEN_HEIGHT / 2.0, // TODO: be an own function that takes half the window size instead of 500
                    0.0,
                ),
                scale: Vec3::new(SCALE, SCALE, 0.0),
                ..Default::default()
            },
            ..Default::default()
        });

        if *tile == TileType::Wall {
            entity.insert(Collidable { x: pos.x, y: pos.y });
        }
    }
}
