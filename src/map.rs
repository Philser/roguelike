use std::fs::File;

use bevy::prelude::*;

use crate::{player::Player, GameState};

const TILE_SIZE: f32 = 24.0;
const SCALE: f32 = 1.0;

pub struct GameMapPlugin {}

impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system()).add_system_set(
            SystemSet::on_enter(GameState::MapLoaded).with_system(render_map.system()),
        );
    }
}

struct GameMap {
    height: i32,
    width: i32,
    tiles: Vec<Vec<TileType>>, //TODO: Make this a HashMap of (x,y), TileTyoe
}

impl GameMap {
    pub fn load_from(level: Level) -> Self {
        let mut tiles: Vec<Vec<TileType>> = vec![];
        for row in level.layout {
            let mut tile_row: Vec<TileType> = vec![];
            for col in row.chars() {
                match col {
                    '#' => {
                        tile_row.push(TileType::Wall);
                    }
                    '.' => {
                        tile_row.push(TileType::Floor);
                    }
                    '@' => { /* Ignore player as they are no tile */ }
                    unknown => panic!("Couldn't parse map due to unknown character: {}", unknown),
                }
            }
            tiles.push(tile_row);
        }

        tiles.reverse();
        GameMap {
            height: tiles.len() as i32,
            width: tiles
                .first()
                .ok_or("Error loading Map: Map does not have tiles")
                .unwrap()
                .len() as i32,
            tiles,
        }
    }
}

pub struct Materials {
    pub player: Handle<ColorMaterial>,
    pub wall: Handle<ColorMaterial>,
    pub hostile: Handle<ColorMaterial>,
    pub friendly: Handle<ColorMaterial>,
    pub floor: Handle<ColorMaterial>,
}

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
    let mut tiles: Vec<Vec<TileType>> = vec![];
    for (y, row) in level.layout.iter().enumerate() {
        let mut tile_row: Vec<TileType> = vec![];
        for (x, col) in row.chars().enumerate() {
            match col {
                '#' => {
                    tile_row.push(TileType::Wall);
                }
                '.' => {
                    tile_row.push(TileType::Floor);
                }
                '@' => {
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
                                    (x as f32 - 10.0) * TILE_SIZE, // TODO: Right now I am lazy but this def. needs to
                                    (y as f32 - 10.0) * TILE_SIZE, // TODO: be an own function that takes half the window size instead of 500
                                    0.0,
                                ),
                                scale: Vec3::new(SCALE, SCALE, 0.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Player {
                            x: x as i32,
                            y: y as i32,
                        });
                }
                unknown => panic!("Couldn't parse map due to unknown character: {}", unknown),
            }
        }
        tiles.push(tile_row);
    }

    tiles.reverse();
    GameMap {
        height: tiles.len() as i32,
        width: tiles
            .first()
            .ok_or("Error loading Map: Map does not have tiles")
            .unwrap()
            .len() as i32,
        tiles,
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut app_state: ResMut<State<GameState>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let level1_file = File::open("assets/levels/level1.ron").expect("Could not load level 1");
    let level1 = match ron::de::from_reader(level1_file) {
        Ok(lvl) => lvl,
        Err(e) => panic!("Error deserializing RON  file: {}", e),
    };

    let materials = Materials {
        player: materials.add(Color::rgb_u8(0, 163, 204).into()),
        wall: materials.add(Color::rgb_u8(217, 217, 217).into()),
        hostile: materials.add(Color::rgb_u8(204, 41, 0).into()),
        friendly: materials.add(Color::rgb_u8(51, 255, 178).into()),
        floor: materials.add(Color::rgb(0.01, 0.01, 0.12).into()),
    };

    let map = parse_level(&mut commands, &materials, level1);

    commands.insert_resource(map);

    commands.insert_resource(materials);
    app_state.set(GameState::MapLoaded).unwrap();
}

fn render_map(mut commands: Commands, map: Res<GameMap>, materials: Res<Materials>) {
    for (y, row) in map.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let material: Handle<ColorMaterial>;
            match tile {
                TileType::Floor => material = materials.floor.clone(),
                TileType::Wall => material = materials.wall.clone(),
            };

            commands.spawn().insert_bundle(SpriteBundle {
                sprite: Sprite {
                    size: Vec2::new(TILE_SIZE * SCALE, TILE_SIZE * SCALE),
                    ..Default::default()
                },
                material: material.clone(),
                transform: Transform {
                    translation: Vec3::new(
                        (x as f32 - 10.0) * TILE_SIZE, // TODO: Right now I am lazy but this def. needs to
                        (y as f32 - 10.0) * TILE_SIZE, // TODO: be an own function that takes half the window size instead of 500
                        0.0,
                    ),
                    scale: Vec3::new(SCALE, SCALE, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}
