use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Item {
    pub name: String,
    pub image: String,
    pub pixelated: bool,
    pub description: String,
}
