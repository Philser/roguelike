use bevy::prelude::Component;

#[derive(Component)]
pub struct Item {}

pub const DEFAULT_HEALTH_POTION_HEAL: i32 = 20;

#[derive(Component)]
pub struct HealthPotion {
    pub heal_amount: i32,
}
