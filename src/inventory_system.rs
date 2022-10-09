use bevy::prelude::*;

use crate::{components::item::Item, GameState};

pub struct InventorySystemPlugin {}

impl Plugin for InventorySystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::PlayerTurn).with_system(pickup_handler));
    }
}

#[derive(Component)]
pub struct WantsToPickupItem {
    pub entity: Entity,
    pub item: Item,
}

fn pickup_handler(pickup_query: Query<&WantsToPickupItem>) {
    // register key stroke
    // see if item is available at player position
    // add item to inventory
    // remove item from map
}
