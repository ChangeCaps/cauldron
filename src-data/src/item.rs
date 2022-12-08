use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ItemDescriptor {
    pub name: String,
    pub id: Option<String>,
    pub image: Option<String>,
    pub pixelated: bool,
    pub categories: Vec<String>,
    pub description: String,
}

impl ItemDescriptor {
    pub fn get_id(&self) -> &str {
        self.id.as_deref().unwrap_or(&self.name)
    }

    pub fn get_image(&self) -> &str {
        self.image.as_deref().unwrap_or("")
    }
}
