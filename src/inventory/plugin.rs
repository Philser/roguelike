use bevy::prelude::{App, Plugin, SystemSet};

use super::systems::{inventory_renderer, pickup_handler, user_input_handler};
use crate::GameState;

/// Plugin for handling all the inventory logic, including rendering, navigating and using items.
pub struct InventorySystemPlugin {}

impl Plugin for InventorySystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::PlayerTurn).with_system(pickup_handler));
        app.add_system_set(
            SystemSet::on_update(GameState::AwaitingInventoryInput).with_system(user_input_handler),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::RenderInventory).with_system(inventory_renderer),
        );
    }
}
