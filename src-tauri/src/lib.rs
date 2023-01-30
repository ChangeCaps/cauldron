#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod categories;
mod category;
mod item;
mod items;

pub use categories::*;
pub use category::*;
pub use item::*;
pub use items::*;

pub fn run() {
    tauri::Builder::default()
        .manage(Items::default())
        .manage(Categories::default())
        .invoke_handler(tauri::generate_handler![
            search_items,
            new_item,
            remove_item,
            get_item,
            set_item,
            load_items,
            store_items,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(mobile)]
#[tauri::mobile_entry_point]
fn main() {
    run();
}
