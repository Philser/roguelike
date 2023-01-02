use bevy::{prelude::*, utils::HashSet};
use rand::{prelude::ThreadRng, Rng};

use crate::{
    components::{
        collidable::Collidable,
        combat_stats::CombatStats,
        item::{Heals, Item},
    },
    components::{
        consumable::Consumable,
        damage::InflictsDamage,
        item::{ItemName, Ranged},
        position::Position,
    },
    configs::game_settings::GameplaySettings,
    inventory::components::Inventory,
    monster::{Monster, MONSTER_FOV},
    player::{Player, PLAYER_FOV},
    utils::{rectangle::Rectangle, render::map_pos_to_screen_pos},
    viewshed::Viewshed,
    ScreenDimensions, TileProperties,
};

const MAX_MONSTERS_PER_ROOM: usize = 2;
const INVENTORY_SIZE: usize = 4;

pub fn spawn_player(
    commands: &mut Commands,
    pos: Position,
    tile_properties: &TileProperties,
    screen_dimensions: &ScreenDimensions,
    gameplay_settings: &GameplaySettings,
) {
    let scaled_tilesize = tile_properties.get_scaled_tile_size();
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(0, 163, 204).into(),
                custom_size: Some(Vec2::new(scaled_tilesize, scaled_tilesize)),
                ..Default::default()
            },
            transform: Transform {
                translation: map_pos_to_screen_pos(
                    &pos,
                    tile_properties.player_z,
                    tile_properties.tile_size,
                    screen_dimensions,
                ),
                scale: Vec3::new(
                    tile_properties.tile_scale,
                    tile_properties.tile_scale,
                    tile_properties.player_z,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(pos)
        .insert(CombatStats {
            hp: gameplay_settings.player_starting_health,
            max_hp: gameplay_settings.player_starting_health,
            defense: 0,
            power: 5,
        })
        .insert(Viewshed {
            visible_tiles: vec![],
            range: PLAYER_FOV,
            dirty: true,
        })
        .insert(Player {})
        .insert(Collidable {})
        .insert(Inventory::new(INVENTORY_SIZE));
}

pub fn spawn_room(
    commands: &mut Commands,
    room: &Rectangle,
    rng: &mut ThreadRng,
    tile_properties: &TileProperties,
    screen_dimensions: &ScreenDimensions,
    gameplay_settings: &GameplaySettings,
) {
    let mut blocked_positions: HashSet<Position> = HashSet::new();

    let monster_count = rng.gen_range(0..=MAX_MONSTERS_PER_ROOM);
    for _ in 0..monster_count {
        match try_find_unblocked_position_in_room(room, &blocked_positions, rng) {
            Some(pos) => {
                spawn_monster(
                    commands,
                    &pos,
                    tile_properties,
                    screen_dimensions,
                    gameplay_settings,
                );
                blocked_positions.insert(pos);
            }
            None => {
                panic!("Room generation failed: Less positions in room than monsters to spawn")
            }
        }
    }

    match try_find_unblocked_position_in_room(room, &blocked_positions, rng) {
        Some(pos) => {
            let item_rng: u32 = rng.gen_range(0..=1);
            match item_rng {
                0 => spawn_health_pot(
                    commands,
                    pos.clone(),
                    tile_properties,
                    screen_dimensions,
                    gameplay_settings,
                ),
                _ => spawn_magic_missle_scroll(
                    commands,
                    pos.clone(),
                    tile_properties,
                    screen_dimensions,
                ),
            }
            blocked_positions.insert(pos);
        }
        None => {
            panic!("Room generation failed: Less positions in room than items to spawn")
        }
    }
}

pub fn spawn_monster(
    commands: &mut Commands,
    pos: &Position,
    tile_properties: &TileProperties,
    screen_dimensions: &ScreenDimensions,
    gameplay_settings: &GameplaySettings,
) {
    let scaled_tile_size = tile_properties.get_scaled_tile_size();
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(204, 41, 0).into(),
                custom_size: Some(Vec2::new(scaled_tile_size, scaled_tile_size)),
                ..Default::default()
            },
            transform: Transform {
                translation: map_pos_to_screen_pos(
                    pos,
                    tile_properties.monster_z,
                    tile_properties.tile_size,
                    screen_dimensions,
                ),
                scale: Vec3::new(
                    tile_properties.tile_scale,
                    tile_properties.tile_scale,
                    tile_properties.monster_z,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position {
            x: pos.x as i32,
            y: pos.y as i32,
        })
        .insert(CombatStats {
            hp: gameplay_settings.monster_starting_health,
            max_hp: gameplay_settings.monster_starting_health,
            defense: 0,
            power: 2,
        })
        .insert(Viewshed {
            visible_tiles: vec![],
            range: MONSTER_FOV,
            dirty: true,
        })
        .insert(Collidable {})
        .insert(Monster {});
}

pub fn spawn_health_pot(
    commands: &mut Commands,
    pos: Position,
    tile_properties: &TileProperties,
    screen_dimensions: &ScreenDimensions,
    gameplay_settings: &GameplaySettings,
) {
    let scaled_tile_size = tile_properties.get_scaled_tile_size();
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(34, 139, 34).into(),
                custom_size: Some(Vec2::new(scaled_tile_size, scaled_tile_size)),
                ..Default::default()
            },
            transform: Transform {
                translation: map_pos_to_screen_pos(
                    &pos,
                    tile_properties.item_z,
                    tile_properties.tile_size,
                    screen_dimensions,
                ),
                scale: Vec3::new(
                    tile_properties.tile_scale,
                    tile_properties.tile_scale,
                    tile_properties.item_z,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position {
            x: pos.x as i32,
            y: pos.y as i32,
        })
        .insert(Item {})
        .insert(ItemName {
            name: "Health Potion".to_owned(),
        })
        .insert(Heals {
            heal_amount: gameplay_settings.health_potion_heal_amount,
        })
        .insert(Consumable {});
}

pub fn spawn_magic_missle_scroll(
    commands: &mut Commands,
    pos: Position,
    tile_properties: &TileProperties,
    screen_dimensions: &ScreenDimensions,
) {
    let scaled_tile_size = tile_properties.get_scaled_tile_size();
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(227, 23, 224).into(),
                custom_size: Some(Vec2::new(scaled_tile_size, scaled_tile_size)),
                ..Default::default()
            },
            transform: Transform {
                translation: map_pos_to_screen_pos(
                    &pos,
                    tile_properties.item_z,
                    tile_properties.tile_size,
                    screen_dimensions,
                ),
                scale: Vec3::new(
                    tile_properties.tile_scale,
                    tile_properties.tile_scale,
                    tile_properties.item_z,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position {
            x: pos.x as i32,
            y: pos.y as i32,
        })
        .insert(Item {})
        .insert(ItemName {
            name: "Magic Missile Scroll".to_owned(),
        })
        .insert(InflictsDamage { damage: 8 })
        .insert(Ranged { range: 6 })
        .insert(Consumable {});
}

fn try_find_unblocked_position_in_room(
    room: &Rectangle,
    blocked_positions: &HashSet<Position>,
    rng: &mut ThreadRng,
) -> Option<Position> {
    let pos_count = room.width() * room.height();

    if blocked_positions.len() as i32 >= pos_count {
        return None;
    }

    let mut pos: Position;
    loop {
        let pos_x = rng.gen_range(room.x1..=room.x2);
        let pos_y = rng.gen_range(room.y1..=room.y2);

        pos = Position { x: pos_x, y: pos_y };
        if !blocked_positions.contains(&pos) {
            break;
        }
    }

    Some(pos)
}
