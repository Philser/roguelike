use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
};

use bevy::prelude::*;
use rand::{prelude::ThreadRng, Rng};

use crate::{
    components::position::Position,
    configs::game_settings::GameplaySettings,
    player::Player,
    spawner::{self, spawn_player},
    utils::{rectangle::Rectangle, render::map_pos_to_screen_pos},
    viewshed::Viewshed,
    GameConfig, GameState, MapProperties, ScreenDimensions, TileProperties,
};

pub struct GameMapPlugin {}

impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_set(SystemSet::on_enter(GameState::MapLoaded).with_system(spawn_map_tiles))
            .add_system_set(
                SystemSet::on_update(GameState::Render)
                    .with_system(render_map)
                    .label("render_map"),
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
    pub blocked_tiles: HashSet<Position>,
    pub tile_content: HashMap<Position, Entity>,
}

impl GameMap {
    pub fn new(
        height: i32,
        width: i32,
        tiles: HashMap<Position, TileType>,
        visited_tiles: HashSet<Position>,
        blocked_tiles: HashSet<Position>,
        tile_content: HashMap<Position, Entity>,
    ) -> Self {
        GameMap {
            height,
            width,
            tiles,
            visited_tiles,
            blocked_tiles,
            tile_content,
        }
    }

    pub fn get_traversable_neighbours_with_distance(
        &self,
        position: &Position,
    ) -> Vec<(Position, i32)> {
        vec![
            (position.x - 1, position.y),
            (position.x + 1, position.y),
            (position.x, position.y + 1),
            (position.x, position.y - 1),
        ]
        .into_iter()
        .map(|p| (Position::new(p.0, p.1), 1))
        .filter(|p| !self.is_blocked(&p.0))
        .collect()
    }

    /// Determines whether a given point in the map is occupied (monsters, player, walls)
    pub fn is_blocked(&self, position: &Position) -> bool {
        if position.x < 0 || position.x >= self.width || position.y < 0 || position.y >= self.height
        {
            return true;
        }

        self.blocked_tiles.get(position).is_some()
    }

    pub fn set_traversable(&mut self, pos: &Position) {
        self.blocked_tiles.remove(pos);
    }

    pub fn set_blocked(&mut self, pos: Position) {
        self.blocked_tiles.insert(pos);
    }

    pub fn remove_blocked(&mut self, pos: &Position) {
        self.blocked_tiles.remove(pos);
    }

    pub fn set_tile_content(&mut self, pos: Position, entity: Entity) {
        self.tile_content.insert(pos, entity);
    }

    pub fn remove_tile_content(&mut self, pos: &Position) {
        self.tile_content.remove(pos);
    }
}

#[derive(Clone)]
pub struct MaterialHandles {
    pub player: Handle<ColorMaterial>,
    pub wall: Handle<ColorMaterial>,
    pub wall_out_of_sight: Handle<ColorMaterial>,
    pub health_potion: Handle<ColorMaterial>,
    pub monster: Handle<ColorMaterial>,
    pub friendly: Handle<ColorMaterial>,
    pub floor: Handle<ColorMaterial>,
    pub floor_out_of_sight: Handle<ColorMaterial>,
}

#[derive(PartialEq, Eq, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Component)]
pub struct Tile {}

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

/// Generate the world map by randomly generating rooms
fn generate_map(
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

#[derive(Component)]
pub struct MainCamera {}

/// Generate the map, load materials and spawn the camera.
/// Sets the game to `GameState::MapLoaded` when done
fn setup(
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

    let map = generate_map(
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

/// Render everything that is visible to the player in the world, i.e. tiles, monsters, and the player
fn render_map(
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

/// Iterate over the game map and spawn a tile with the proper material for each cell of the map
fn spawn_map_tiles(
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
