use bevy::{prelude::*, utils::HashSet};
use rand::{prelude::ThreadRng, Rng};

use crate::{
    components::{
        collidable::Collidable,
        combat_stats::CombatStats,
        item::{Heals, Item, DEFAULT_HEALTH_POTION_HEAL},
    },
    components::{
        consumable::Consumable,
        damage::InflictsDamage,
        item::{ItemName, Ranged},
        position::Position,
    },
    inventory::components::Inventory,
    monster::{Monster, MONSTER_FOV, MONSTER_STARTING_HEALTH},
    player::{Player, PLAYER_FOV, PLAYER_STARTING_HEALTH},
    utils::{rectangle::Rectangle, render::map_pos_to_screen_pos},
    viewshed::Viewshed,
};

const MAX_MONSTERS_PER_ROOM: usize = 2;
const INVENTORY_SIZE: usize = 4;

pub fn spawn_player(
    commands: &mut Commands,
    pos: Position,
    tile_size: f32,
    scale: f32,
    player_z: f32,
    screen_width: f32,
    screen_height: f32,
) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(0, 163, 204).into(),
                custom_size: Some(Vec2::new(tile_size * scale, tile_size * scale)),
                ..Default::default()
            },
            transform: Transform {
                translation: map_pos_to_screen_pos(
                    &pos,
                    player_z,
                    tile_size,
                    screen_width,
                    screen_height,
                ),
                scale: Vec3::new(scale, scale, player_z),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(pos)
        .insert(CombatStats {
            hp: PLAYER_STARTING_HEALTH,
            max_hp: PLAYER_STARTING_HEALTH,
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
    tile_size: f32,
    scale: f32,
    monster_z: f32,
    item_z: f32,
    screen_width: f32,
    screen_height: f32,
) {
    let mut blocked_positions: HashSet<Position> = HashSet::new();

    let monster_count = rng.gen_range(0..=MAX_MONSTERS_PER_ROOM);
    for _ in 0..monster_count {
        match try_find_unblocked_position_in_room(room, &blocked_positions, rng) {
            Some(pos) => {
                spawn_monster(
                    commands,
                    &pos,
                    tile_size,
                    scale,
                    monster_z,
                    screen_width,
                    screen_height,
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
                    tile_size,
                    scale,
                    item_z,
                    screen_width,
                    screen_height,
                ),
                _ => spawn_magic_missle_scroll(
                    commands,
                    pos.clone(),
                    tile_size,
                    scale,
                    item_z,
                    screen_width,
                    screen_height,
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
    tile_size: f32,
    scale: f32,
    monster_z: f32,
    screen_width: f32,
    screen_height: f32,
) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(204, 41, 0).into(),
                custom_size: Some(Vec2::new(tile_size * scale, tile_size * scale)),
                ..Default::default()
            },
            transform: Transform {
                translation: map_pos_to_screen_pos(
                    pos,
                    monster_z,
                    tile_size,
                    screen_width,
                    screen_height,
                ),
                scale: Vec3::new(scale, scale, monster_z),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position {
            x: pos.x as i32,
            y: pos.y as i32,
        })
        .insert(CombatStats {
            hp: MONSTER_STARTING_HEALTH,
            max_hp: MONSTER_STARTING_HEALTH,
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
    tile_size: f32,
    scale: f32,
    item_z: f32,
    screen_width: f32,
    screen_height: f32,
) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(34, 139, 34).into(),
                custom_size: Some(Vec2::new(tile_size * scale, tile_size * scale)),
                ..Default::default()
            },
            transform: Transform {
                translation: map_pos_to_screen_pos(
                    &pos,
                    item_z,
                    tile_size,
                    screen_width,
                    screen_height,
                ),
                scale: Vec3::new(scale, scale, item_z),
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
            heal_amount: DEFAULT_HEALTH_POTION_HEAL,
        })
        .insert(Consumable {});
}

pub fn spawn_magic_missle_scroll(
    commands: &mut Commands,
    pos: Position,
    tile_size: f32,
    scale: f32,
    item_z: f32,
    screen_width: f32,
    screen_height: f32,
) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(227, 23, 224).into(),
                custom_size: Some(Vec2::new(tile_size * scale, tile_size * scale)),
                ..Default::default()
            },
            transform: Transform {
                translation: map_pos_to_screen_pos(
                    &pos,
                    item_z,
                    tile_size,
                    screen_width,
                    screen_height,
                ),
                scale: Vec3::new(scale, scale, item_z),
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
