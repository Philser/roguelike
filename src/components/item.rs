use bevy::prelude::Component;

#[derive(Hash, Clone, PartialEq, Eq)]
pub enum ItemType {
    HealthPotion,
}

//TODO: Introduce item type
#[derive(Component, Clone)]
pub struct Item {
    pub item_type: ItemType,
}

pub const DEFAULT_HEALTH_POTION_HEAL: i32 = 20;

#[derive(Component)]
pub struct HealthPotion {
    pub heal_amount: i32,
}
