use std::{collections::HashMap, fs, path::Path};

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use uuid::Uuid;

use crate::Item;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Items {
    pub items: DashMap<Uuid, Item>,
}

#[tauri::command]
pub fn search_items(items: tauri::State<Items>, query: String) -> Vec<Uuid> {
    let mut result = Vec::new();

    for item in items.items.iter() {
        if query.is_empty() {
            result.push((*item.key(), 0));
            continue;
        }

        let Some(best) = sublime_fuzzy::best_match(&query, &item.name) else {
            continue;
        };

        result.push((*item.key(), best.score()));
    }

    result.sort_by_key(|(_, score)| *score);
    result.into_iter().map(|(key, _)| key).collect()
}

#[tauri::command]
pub fn new_item(items: tauri::State<Items>) -> Uuid {
    let id = Uuid::new_v4();
    items.items.insert(id, Item::default());
    id
}

#[tauri::command]
pub fn remove_item(app: tauri::AppHandle, items: tauri::State<Items>, item: Uuid) {
    items.items.remove(&item);

    app.emit_all("items-changed", ()).unwrap();
}

#[tauri::command]
pub fn get_item(items: tauri::State<Items>, item: Uuid) -> Option<Item> {
    Some(items.items.get(&item)?.clone())
}

#[tauri::command]
pub fn set_item(app: tauri::AppHandle, items: tauri::State<Items>, item: Uuid, value: Item) {
    items.items.insert(item, value);

    app.emit_all("item-changed", item).unwrap();
}

#[tauri::command]
pub fn store_items(app: tauri::AppHandle, items: tauri::State<Items>, file: Option<String>) {
    let file_name = file.unwrap_or(String::from("items"));
    let file_path = Path::new(&file_name).with_extension("json");

    let path = app.path_resolver().app_data_dir().unwrap().join(file_path);
    let json = serde_json::to_string(&items.items).unwrap();
    fs::write(path, json).unwrap();
}

#[tauri::command]
pub fn load_items(app: tauri::AppHandle, items: tauri::State<Items>, file: Option<String>) {
    let file_name = file.unwrap_or(String::from("items"));
    let file_path = Path::new(&file_name).with_extension("json");

    let path = app.path_resolver().app_data_dir().unwrap().join(file_path);
    let Ok(json) = fs::read_to_string(path) else { return };
    let loaded: HashMap<Uuid, Item> = serde_json::from_str(&json).unwrap();

    for (key, value) in loaded {
        items.items.insert(key, value);
    }

    app.emit_all("items-changed", ()).unwrap();
}
