use bevy::prelude::*;

use crate::{
    components::{
        inventory::{Inventory, PlayerInventory},
        item::Item,
    },
    GameState,
};

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

fn pickup_handler(
    pickup_query: Query<&WantsToPickupItem>,
    mut inventory_query: Query<&mut Inventory, With<PlayerInventory>>,
) {
    let mut inventory = inventory_query
        .get_single_mut()
        .expect("We don't have exactly one inventory!!11");

    for pickup_attempt in pickup_query.iter() {
        inventory.add(&pickup_attempt.item.item_type, pickup_attempt.entity);
        
    }
    // add item to inventory
    // remove item from map
}
