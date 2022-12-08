use std::iter;

use cauldron_data::item::ItemDescriptor;
use garlic::{SearchQuery, Style};
use yew::prelude::*;

use crate::{
    data::DataContext, panels::PanelContext, right_panel::RightPanel, search::search_enumerate,
};

#[derive(Properties, PartialEq)]
pub struct ItemViewProps {
    pub item: ItemDescriptor,
    #[prop_or_default]
    pub onclick: Callback<()>,
}

#[function_component]
pub fn ItemView(props: &ItemViewProps) -> Html {
    let mut style = Style::new();

    if props.item.pixelated {
        style.set("filter", "grayscale(100%)");
    }

    html! {
        <div
            class="item-view"
            style={ style }
            title={ props.item.name.clone() }
            onclick={ props.onclick.clone().reform(|_| ()) }
        >
            <img src={ props.item.image.clone() } draggable="false"/>
        </div>
    }
}

#[function_component]
pub fn ItemsPanel() -> Html {
    let panels = use_context::<PanelContext>().unwrap();
    let data = use_context::<DataContext>().unwrap();

    let search_query = use_state(String::new);

    let ref_data = data.borrow();
    let sorted: Vec<_> = if !search_query.is_empty() {
        search_enumerate(&search_query, &ref_data.items, |item| {
            iter::once((0, item.name.as_str()))
                .chain(item.id.iter().map(|id| (-1000, id.as_str())))
                .chain(item.categories.iter().map(|s| (-10000, s.as_str())))
                .chain(iter::once((-100000, item.description.as_str())))
        })
        .collect()
    } else {
        ref_data.items.iter().enumerate().collect()
    };

    let mut items = Vec::with_capacity(sorted.len());
    for (i, item) in sorted {
        let right = panels.right.clone();

        let onclick = Callback::from(move |_| {
            let panel = RightPanel::ItemEditor { index: i };

            right.set(panel);
        });

        items.push(html! {
            <ItemView
                item={ item.clone() }
                onclick={ onclick }
            />
        });
    }

    let add_item = use_callback(
        |_, (panels, data)| {
            let mut data = data.borrow_mut();
            data.items.push(ItemDescriptor::default());

            let panel = RightPanel::ItemEditor {
                index: data.items.len() - 1,
            };

            panels.right.set(panel);
        },
        (panels.clone(), data.clone()),
    );

    html! {
        <>
            <SearchQuery style="width: 100%;" query={ search_query.clone() }/>

            <div class="items-panel">
                { for items }

                <div class="add-item" title="New item" onpointerdown={ add_item }>
                    <img src="https://cdn-icons-png.flaticon.com/512/3161/3161837.png" />
                </div>
            </div>
        </>
    }
}
