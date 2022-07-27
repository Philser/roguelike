use bevy::{prelude::*, utils::HashSet};
use rand::{prelude::ThreadRng, Rng};

use crate::{
    components::{
        collidable::Collidable,
        item::{HealthPotion, Item, DEFAULT_HEALTH_POTION_HEAL},
        CombatStats::CombatStats,
    },
    map::SCALE,
    monster::{Monster, MONSTER_FOV, MONSTER_STARTING_HEALTH},
    player::{Player, PLAYER_FOV, PLAYER_STARTING_HEALTH},
    position::Position,
    utils::{rectangle::Rectangle, render::map_pos_to_screen_pos},
    viewshed::Viewshed,
    ITEM_Z, MONSTER_Z, PLAYER_Z, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE,
};

const MAX_MONSTERS_PER_ROOM: usize = 4;
const MAX_ITEMS_PER_ROOM: usize = 1;

pub fn spawn_player(commands: &mut Commands, color: Color, pos: Position) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color,
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
        .insert(Collidable {});
}

pub fn spawn_room(
    commands: &mut Commands,
    monster_color: Color,
    item_color: Color,
    room: &Rectangle,
    rng: &mut ThreadRng,
) {
    let mut blocked_positions: HashSet<Position> = HashSet::new();

    let monster_count = rng.gen_range(0..=MAX_MONSTERS_PER_ROOM);
    for _ in 0..monster_count {
        let mut pos: Position;
        loop {
            // Try to find a position that is not yet blocked
            // TODO: we could theoretically construct a scenario where there are more monsters than positions
            // and this loop would never exit
            let pos_x = rng.gen_range(room.x1..=room.x2);
            let pos_y = rng.gen_range(room.y1..=room.y2);

            pos = Position { x: pos_x, y: pos_y };
            if !blocked_positions.contains(&pos) {
                break;
            }
        }

        spawn_monster(commands, monster_color, pos.clone());
        blocked_positions.insert(pos);
    }

    let item_count = rng.gen_range(0..=MAX_ITEMS_PER_ROOM);
    for _ in 0..item_count {
        let mut pos: Position;
        loop {
            // Try to find a position that is not yet blocked
            // TODO: we could theoretically construct a scenario where there are more items than positions
            // and this loop would never exit
            let pos_x = rng.gen_range(room.x1..=room.x2);
            let pos_y = rng.gen_range(room.y1..=room.y2);

            pos = Position { x: pos_x, y: pos_y };
            if !blocked_positions.contains(&pos) {
                break;
            }
        }

        spawn_item(commands, item_color, pos.clone());
        blocked_positions.insert(pos);
    }
}

pub fn spawn_monster(commands: &mut Commands, color: Color, pos: Position) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(TILE_SIZE * SCALE, TILE_SIZE * SCALE)),
                ..Default::default()
            },
            transform: Transform {
                translation: map_pos_to_screen_pos(
                    &pos,
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

pub fn spawn_item(commands: &mut Commands, color: Color, pos: Position) {
    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color,
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
        .insert(Item {})
        .insert(HealthPotion {
            heal_amount: DEFAULT_HEALTH_POTION_HEAL,
        });
}
