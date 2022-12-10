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
        item::{ItemType, Ranged},
        position::Position,
    },
    inventory::components::Inventory,
    map::SCALE,
    monster::{Monster, MONSTER_FOV, MONSTER_STARTING_HEALTH},
    player::{Player, PLAYER_FOV, PLAYER_STARTING_HEALTH},
    utils::{rectangle::Rectangle, render::map_pos_to_screen_pos},
    viewshed::Viewshed,
    ITEM_Z, MONSTER_Z, PLAYER_Z, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE,
};

const MAX_MONSTERS_PER_ROOM: usize = 2;
const INVENTORY_SIZE: usize = 4;

pub fn spawn_player(commands: &mut Commands, pos: Position) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(0, 163, 204).into(),
                custom_size: Some(Vec2::new(TILE_SIZE * SCALE, TILE_SIZE * SCALE)),
                ..Default::default()
            },
            transform: Transform {
                translation: map_pos_to_screen_pos(
                    &pos,
                    PLAYER_Z,
                    TILE_SIZE,
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                ),
                scale: Vec3::new(SCALE, SCALE, PLAYER_Z),
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

pub fn spawn_room(commands: &mut Commands, room: &Rectangle, rng: &mut ThreadRng) {
    let mut blocked_positions: HashSet<Position> = HashSet::new();

    let monster_count = rng.gen_range(0..=MAX_MONSTERS_PER_ROOM);
    for _ in 0..monster_count {
        match try_find_unblocked_position_in_room(room, &blocked_positions, rng) {
            Some(pos) => {
                spawn_monster(commands, &pos);
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
                0 => spawn_health_pot(commands, pos.clone()),
                _ => spawn_magic_missle_scroll(commands, pos.clone()),
            }
            blocked_positions.insert(pos);
        }
        None => {
            panic!("Room generation failed: Less positions in room than items to spawn")
        }
    }
}

pub fn spawn_monster(commands: &mut Commands, pos: &Position) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(204, 41, 0).into(),
                custom_size: Some(Vec2::new(TILE_SIZE * SCALE, TILE_SIZE * SCALE)),
                ..Default::default()
            },
            transform: Transform {
                translation: map_pos_to_screen_pos(
                    pos,
                    MONSTER_Z,
                    TILE_SIZE,
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                ),
                scale: Vec3::new(SCALE, SCALE, MONSTER_Z),
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

pub fn spawn_health_pot(commands: &mut Commands, pos: Position) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(34, 139, 34).into(),
                custom_size: Some(Vec2::new(TILE_SIZE * SCALE, TILE_SIZE * SCALE)),
                ..Default::default()
            },
            transform: Transform {
                translation: map_pos_to_screen_pos(
                    &pos,
                    ITEM_Z,
                    TILE_SIZE,
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                ),
                scale: Vec3::new(SCALE, SCALE, ITEM_Z),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position {
            x: pos.x as i32,
            y: pos.y as i32,
        })
        .insert(Item {
            item_type: ItemType::HealthPotion,
        })
        .insert(Heals {
            heal_amount: DEFAULT_HEALTH_POTION_HEAL,
        })
        .insert(Consumable {});
}

pub fn spawn_magic_missle_scroll(commands: &mut Commands, pos: Position) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(227, 23, 224).into(),
                custom_size: Some(Vec2::new(TILE_SIZE * SCALE, TILE_SIZE * SCALE)),
                ..Default::default()
            },
            transform: Transform {
                translation: map_pos_to_screen_pos(
                    &pos,
                    ITEM_Z,
                    TILE_SIZE,
                    SCREEN_WIDTH,
                    SCREEN_HEIGHT,
                ),
                scale: Vec3::new(SCALE, SCALE, ITEM_Z),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Position {
            x: pos.x as i32,
            y: pos.y as i32,
        })
        .insert(Item {
            item_type: ItemType::MagicMissleScroll,
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
