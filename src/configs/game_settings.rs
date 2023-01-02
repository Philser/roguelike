pub struct GameConfig {
    pub tile_properties: TileProperties,
    pub screen_dimensions: ScreenDimensions,
    pub map_properties: MapProperties,
    pub gameplay_settings: GameplaySettings,
}

pub const SCREEN_HEIGHT: f32 = 720.0;
pub const SCREEN_WIDTH: f32 = 1280.0;

pub struct ScreenDimensions {
    pub screen_height: f32,
    pub screen_width: f32,
}

pub struct TileProperties {
    pub tile_size: f32,
    pub tile_scale: f32,
    pub player_z: f32,
    pub monster_z: f32,
    pub item_z: f32,
}

impl TileProperties {
    pub fn get_scaled_tile_size(&self) -> f32 {
        return self.tile_scale * self.tile_size;
    }
}

pub struct MapProperties {
    pub map_height: i32,
    pub map_width: i32,
    pub max_rooms: u32,
}

pub struct GameplaySettings {
    pub player_starting_health: i32,
    pub health_potion_heal_amount: i32,
    pub monster_starting_health: i32,
}
