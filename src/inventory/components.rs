use bevy::prelude::{Component, Entity};

use crate::components::position::Position;
/// Component that holds a vector of items per item type. Used by the inventory plugin.
#[derive(Component)]
pub struct Inventory {
    pub items: Vec<Option<Entity>>,
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
            items.push(None)
        }

        Inventory {
            items,
            inventory_size,
        }
    }

    pub fn add_item(&mut self, item: Entity) -> Result<(), InventoryError> {
        for i in 0..self.inventory_size {
            if self.items[i].is_none() {
                self.items[i] = Some(item);
                return Ok(());
            }
        }

        return Err(InventoryError::InventoryFull);
    }

    pub fn remove_item_by_entity(&mut self, target_entity: Entity) {
        let mut slot_to_empty: Option<usize> = None;
        for (slot, item) in self.items.iter().enumerate() {
            if let Some(item_entity) = item {
                if *item_entity == target_entity {
                    slot_to_empty = Some(slot);
                }
            }
        }

        if let Some(slot) = slot_to_empty {
            self.items[slot] = None
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
    pub item_name: String,
}

/// Component that flags an entity that is an item for pickup to inventory.
#[derive(Component)]
pub struct WantsToUseItem {
    pub entity: Entity,
    pub targets: Option<Vec<Position>>,
}
