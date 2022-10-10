use bevy::prelude::*;

use crate::{
    components::{inventory::Inventory, item::Item, position::Position},
    player::Player,
    user_interface::ActionLog,
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
    mut commands: Commands,
    pickup_query: Query<(Entity, &WantsToPickupItem)>,
    mut player_inventory_query: Query<&mut Inventory, With<Player>>,
    mut action_log: ResMut<ActionLog>,
) {
    let mut player_inv = player_inventory_query
        .get_single_mut()
        .expect("We don't have exactly one inventory!!11");

    for (pickup_attempt_entity, pickup_attempt) in pickup_query.iter() {
        // remove item from map
        commands.entity(pickup_attempt.entity).remove::<Sprite>();
        commands.entity(pickup_attempt.entity).remove::<Transform>();
        commands.entity(pickup_attempt.entity).remove::<Position>();

        player_inv.add(&pickup_attempt.item.item_type, pickup_attempt.entity);

        action_log
            .entries
            .push(format!("Picked up {}", pickup_attempt.item.item_type));

        commands.entity(pickup_attempt_entity).despawn();
    }
}
