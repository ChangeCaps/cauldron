#![allow(dead_code)]

mod data;
mod inventory;
mod item;
mod item_editor;
mod main_panel;
mod panels;
mod right_panel;
mod search;

use yew::prelude::*;

use crate::data::DataLoader;

#[macro_export]
macro_rules! log {
    ($($tt:tt)*) => {
        web_sys::console::log_1(&format_args!($($tt)*).to_string().into());
    };
}

#[function_component]
pub fn App() -> Html {
    html! {
        <DataLoader>
            <panels::Panels />
        </DataLoader>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
