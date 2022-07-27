use bevy::prelude::*;
use rand::{prelude::ThreadRng, Rng};

use crate::{
    components::{collidable::Collidable, CombatStats::CombatStats},
    map::{MAP_HEIGHT, MAP_WIDTH, SCALE},
    monster::{Monster, MONSTER_FOV, MONSTER_STARTING_HEALTH},
    player::{Player, PLAYER_FOV, PLAYER_STARTING_HEALTH},
    position::Position,
    utils::{rectangle::Rectangle, render::map_pos_to_screen_pos},
    viewshed::Viewshed,
    MONSTER_Z, PLAYER_Z, SCREEN_HEIGHT, SCREEN_WIDTH, TILE_SIZE,
};

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

pub fn spawn_room(commands: &mut Commands, color: Color, room: &Rectangle, rng: &mut ThreadRng) {
    // TODO: Add RNG monster spawning (count and position)
    let (x, y) = room.get_center();
    spawn_monster(commands, color, Position { x, y });
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
