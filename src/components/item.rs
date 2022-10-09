use bevy::prelude::Component;

//TODO: Introduce item type
#[derive(Component)]
pub struct Item {}

pub const DEFAULT_HEALTH_POTION_HEAL: i32 = 20;

#[derive(Component)]
pub struct HealthPotion {
    pub heal_amount: i32,
}
