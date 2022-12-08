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
}
