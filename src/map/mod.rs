use bevy::{
    prelude::{Component, Handle, Resource},
    sprite::ColorMaterial,
};

pub mod game_map;
mod generate_map_system;
pub mod plugin;
mod render_map_system;
mod spawn_map_tiles_system;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Clone, Resource, Debug, Default)]
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

#[derive(Component)]
pub struct Tile {}

#[derive(Component)]
pub struct MainCamera {}
