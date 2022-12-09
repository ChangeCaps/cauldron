pub mod inventory;
pub mod item;

use inventory::Inventory;
use item::ItemDescriptor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Data {
    pub inventory: Inventory,
    pub items: Vec<ItemDescriptor>,
    pub removed_items: Vec<ItemDescriptor>,
}

impl Data {
    #[inline]
    pub fn get_index(&self, id: &str) -> Option<usize> {
        self.items.iter().position(|item| item.get_id() == id)
    }
}
