use yew::prelude::*;

use crate::{inventory::InventoryPanel, item::ItemsPanel, panels::PanelContext};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MainPanel {
    Inventory,
    Items,
}

impl MainPanel {
    pub fn html(&self) -> Html {
        match self {
            MainPanel::Inventory => html! {
                <InventoryPanel />
            },
            MainPanel::Items => html! {
                <ItemsPanel />
            },
        }
    }
}

#[function_component]
pub fn MainPanelView() -> Html {
    let panels = use_context::<PanelContext>().unwrap();

    html! {
        <div class={ classes!("main-panel", "panel-content") }>
            { panels.main.html() }
        </div>
    }
}
