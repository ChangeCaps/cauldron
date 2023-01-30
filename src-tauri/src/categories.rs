use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::Category;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Categories {
    pub categories: DashMap<Uuid, Category>,
}
