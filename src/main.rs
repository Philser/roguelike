use std::fs::File;

use bevy::prelude::*;

const TILE_SIZE: f32 = 32.0;
const SCALE: f32 = 1.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    LoadingResources,
    LoadingFinished,
}

struct Player {
    pub x: i32,
    pub y: i32,
}

struct GameMap {
    height: i32,
    width: i32,
    tiles: Vec<Vec<TileType>>,
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

struct Materials {
    player: Handle<ColorMaterial>,
    wall: Handle<ColorMaterial>,
    hostile: Handle<ColorMaterial>,
    friendly: Handle<ColorMaterial>,
    floor: Handle<ColorMaterial>,
}

enum TileType {
    Wall,
    Floor,
}
#[derive(Debug, serde::Deserialize)]
pub struct Level {
    layout: Vec<String>,
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

    let map = GameMap::load_from(level1);

    commands.insert_resource(map);

    commands.insert_resource(Materials {
        player: materials.add(Color::rgb_u8(0, 163, 204).into()),
        wall: materials.add(Color::rgb_u8(217, 217, 217).into()),
        hostile: materials.add(Color::rgb_u8(204, 41, 0).into()),
        friendly: materials.add(Color::rgb_u8(51, 255, 178).into()),
        floor: materials.add(Color::rgb(0.01, 0.01, 0.12).into()),
    });
    app_state.set(GameState::LoadingFinished).unwrap();
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
                    translation: Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0),
                    scale: Vec3::new(SCALE, SCALE, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        }
    }
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: 1280.0,
            height: 720.0,
            title: "AAAAAAAAHHHHH".to_owned(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_state(GameState::LoadingResources)
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::on_enter(GameState::LoadingFinished).with_system(render_map.system()),
        )
        .run();
}
