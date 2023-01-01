use crate::components::position::Position;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use super::TileType;


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
