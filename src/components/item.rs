use std::fmt;

use bevy::prelude::Component;

#[derive(Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemType {
    HealthPotion,
    MagicMissileScroll,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            ItemType::HealthPotion => "Health Potion",
            ItemType::MagicMissileScroll => "Magic Missle Scroll",
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
pub struct Heals {
    pub heal_amount: i32,
}

#[derive(Component, Debug)]
pub struct Ranged {
    pub range: i32,
}

pub const UNKNOWN_ITEM_NAME: &str = "<NOT IMPLEMENTED>";
#[derive(Component, Debug, Clone)]
pub struct ItemName {
    pub name: String,
}
