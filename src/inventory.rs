use garlic::SearchQuery;
use yew::prelude::*;

use crate::{data::DataContext, item::ItemView, panels::PanelContext};

#[function_component]
pub fn InventoryPanel() -> Html {
    let panels = use_context::<PanelContext>().unwrap();
    let data = use_context::<DataContext>().unwrap();

    let search_query = use_state(String::new);

    let mut items = Vec::with_capacity(data.borrow().items.len());
    for (i, item) in data.borrow().inventory.items.iter().enumerate() {
        let Some(descriptor_index) = data.borrow().get_index(&item.descriptor) else {
            continue;
        };

        let descriptor = &data.borrow().items[descriptor_index];

        items.push(html! {
            <ItemView
                item={ descriptor.clone() }
            />
        });
    }

    html! {
        <>
            <SearchQuery style="width: 100%" query={ search_query.clone() } />

            <div class="items-panel">
                { for items }
            </div>
        </>
    }
}
