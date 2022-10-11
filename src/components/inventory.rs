use std::collections::HashMap;

use bevy::prelude::{Component, Entity};

use super::item::ItemType;

#[derive(Component)]
pub struct Inventory {
    items: HashMap<ItemType, Vec<Entity>>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            items: HashMap::new(),
        }
    }

    pub fn add(&mut self, item_type: &ItemType, entity: Entity) {
        if !self.items.contains_key(item_type) {
            self.items.insert(item_type.clone(), vec![entity]);
        } else {
            let entry = self
                .items
                .get_mut(item_type)
                .expect("This should never happen");

            entry.push(entity);
        }
    }
}

#[derive(Component)]
pub struct PlayerInventory {}
