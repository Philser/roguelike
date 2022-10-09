use bevy::prelude::Component;

use super::item::Item;

#[derive(Component)]
pub struct Backpack {
    pub items: Vec<Item>,
}

impl Backpack {
    pub fn new() -> Backpack {
        Backpack { items: vec![] }
    }
}
