use bevy::math::Vec3;

use crate::position::Position;

// Translate the Game Map position to the screen position
// The Game Map is a coordinate system starting from the bottom left, whereas the screen has it's base in the center
pub fn map_pos_to_screen_pos(
    map_pos: &Position,
    z_coord: f32,
    tile_size: f32,
    screen_width: f32,
    screen_height: f32,
) -> Vec3 {
    Vec3::new(
        map_pos.x as f32 * tile_size - screen_width / 2.0,
        map_pos.y as f32 * tile_size - screen_height / 2.0 + screen_height * 0.2, // leave room for the UI
        z_coord,
    )
}
