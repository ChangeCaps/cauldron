use serde::{Deserialize, Serialize};

use crate::item::ItemDescriptor;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Item {
    pub descriptor: String,
    pub quantity: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Inventory {
    pub items: Vec<ItemDescriptor>,
}
