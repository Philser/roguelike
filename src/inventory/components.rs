use std::collections::HashMap;

use bevy::prelude::{Component, Entity};

use crate::components::{
    item::{Item, ItemType},
    position::Position,
};

/// Component that holds a vector of items per item type. Used by the inventory plugin.
#[derive(Component)]
pub struct Inventory {
    pub items: HashMap<ItemType, Vec<Entity>>,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            items: HashMap::new(),
        }
    }

    /// Add an item's entity to the inventory.
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

/// Flag component for marking the player's inventory.
#[derive(Component)]
pub struct PlayerInventory {}

/// Flag component marking the root frame that all UI elements are children of.
#[derive(Component)]
pub struct InventoryUIRoot {}

/// Flag component marking a single slot in the 2d inventory grid.
#[derive(Component)]
pub struct InventoryUISlot {}

/// Flag component marking the frame for a slot. Used by the inventory cursor to highlight
/// current selection.
#[derive(Component)]
pub struct InventoryUISlotFrame {}

/// Component storing the current position of the cursor in the 2d inventory grid.
/// Also stores all InventoryUISlot entities in a 2d vector that represents the UI.
#[derive(Component)]
pub struct InventoryCursor {
    pub cursor_position: Position,
    pub ui_cursor_slots: Vec<Vec<Entity>>,
}

impl InventoryCursor {
    /// Moves the cursor to the new position, if the new position is within the bounds of the
    /// component's inventory grid.
    pub fn move_cursor(&mut self, x: i32, y: i32) {
        let new_x = self.cursor_position.x + x;
        let new_y = self.cursor_position.y + y;

        if new_y >= 0 && self.ui_cursor_slots.len() > new_y as usize {
            self.cursor_position.y = new_y;
        }

        if new_x >= 0
            && self.ui_cursor_slots[self.cursor_position.y as usize].len() > new_x as usize
        {
            self.cursor_position.x = new_x;
        }
    }
}

/// Component that flags an entity that is an item for pickup to inventory.
#[derive(Component)]
pub struct WantsToPickupItem {
    pub entity: Entity,
    pub item: Item,
}
