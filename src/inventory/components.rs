use bevy::prelude::{Component, Entity};

use crate::components::item::{Item, ItemType};

/// Component that holds a vector of items per item type. Used by the inventory plugin.
#[derive(Component)]
pub struct Inventory {
    pub items: Vec<(ItemType, Option<Entity>)>,
    pub inventory_size: usize,
}
#[derive(PartialEq)]
pub enum InventoryError {
    InventoryFull,
}

impl Inventory {
    pub fn new(inventory_size: usize) -> Inventory {
        let mut items = vec![];
        for _ in 0..inventory_size {
            items.push((ItemType::Nothing, None))
        }

        Inventory {
            items,
            inventory_size,
        }
    }

    pub fn add_item(&mut self, item: (ItemType, Option<Entity>)) -> Result<(), InventoryError> {
        for i in 0..self.inventory_size {
            if self.items[i].0 == ItemType::Nothing {
                self.items[i] = item;
                return Ok(());
            }
        }

        return Err(InventoryError::InventoryFull);
    }

    /// Removes item on position, if one happens to be there.
    /// In case of invalid positions or slots that are already empty, nothing happens.
    pub fn remove_item(&mut self, position: usize) {
        if self.items.len() > position {
            self.items[position] = (ItemType::Nothing, None);
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
    pub cursor_position: usize,
    inventory_size: usize,
}

impl InventoryCursor {
    pub fn new(start_position: usize, inventory_size: usize) -> InventoryCursor {
        InventoryCursor {
            cursor_position: start_position,
            inventory_size,
        }
    }

    /// Moves the cursor to the new position, if the new position is within the bounds of the
    /// component's inventory bar.
    pub fn move_cursor(&mut self, y: i32) {
        let new_y = self.cursor_position as i32 - y;

        if new_y >= 0 && self.inventory_size > new_y as usize {
            self.cursor_position = new_y as usize;
        }
    }
}

/// Component that flags an entity that is an item for pickup to inventory.
#[derive(Component)]
pub struct WantsToPickupItem {
    pub entity: Entity,
    pub item: Item,
}
