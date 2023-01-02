use bevy::prelude::Component;

/// Flag component to indicate an item
#[derive(Component, Clone)]
pub struct Item {}

#[derive(Component)]
pub struct Heals {
    pub heal_amount: i32,
}

#[derive(Component, Debug)]
pub struct Ranged {
    pub range: u32,
}

#[derive(Component, Debug)]
pub struct AreaOfEffect {
    pub radius: u32,
}

pub const UNKNOWN_ITEM_NAME: &str = "<NOT IMPLEMENTED>";
#[derive(Component, Debug, Clone)]
pub struct ItemName {
    pub name: String,
}
