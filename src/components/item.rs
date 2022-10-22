use std::fmt;

use bevy::prelude::Component;

#[derive(Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemType {
    HealthPotion,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            ItemType::HealthPotion => "Health Potion",
        };
        write!(f, "{}", string)
    }
}

#[derive(Component, Clone)]
pub struct Item {
    pub item_type: ItemType,
}

pub const DEFAULT_HEALTH_POTION_HEAL: i32 = 20;

#[derive(Component)]
pub struct HealthPotion {
    pub heal_amount: i32,
}
