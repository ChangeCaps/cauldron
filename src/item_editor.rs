use garlic::{AnimationFrame, Direction, Spacer, TextInput};
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::{data::DataContext, item::ItemView, panels::PanelContext, right_panel::RightPanel};

#[derive(Properties, PartialEq)]
pub struct ItemEditorProps {
    pub index: usize,
}

fn text_input(oninput: Callback<String>, name: &'static str, value: &str) -> Html {
    html! {
        <TextInput
            style="width: 100%;"
            placeholder={ name }
            title={ name }
            oninput={ oninput }
            value={ value.to_string() }
        />
    }
}

fn category_input(oninput: Callback<InputEvent>, value: &str) -> Html {
    html! {
        <input
            type="text"
            style="width: 100%;"
            placeholder="Category"
            title="Category"
            oninput={ oninput }
            value={ value.to_string() }
        />
    }
}

fn new_category(onkeypress: Callback<KeyboardEvent>, value: &str) -> Html {
    html! {
        <input
            type="text"
            style="width: 100%;"
            placeholder="New Category"
            title="New Category"
            onkeypress={ onkeypress }
            value={ value.to_string() }
        />
    }
}

#[function_component]
pub fn ItemEditor(props: &ItemEditorProps) -> Html {
    let panels = use_context::<PanelContext>().unwrap();
    let data = use_context::<DataContext>().unwrap();
    let item = data.borrow().items[props.index].clone();

    let input_name = use_callback(
        move |value, (data, panels, index)| {
            data.borrow_mut().items[*index].name = value;
            data.store();

            panels.update();
        },
        (data.clone(), panels.clone(), props.index),
    );

    let input_id = use_callback(
        move |value, (data, panels, index)| {
            data.borrow_mut().items[*index].id = Some(value);
            data.store();

            panels.update();
        },
        (data.clone(), panels.clone(), props.index),
    );

    let input_image = use_callback(
        move |value, (data, panels, index)| {
            data.borrow_mut().items[*index].image = Some(value);
            data.store();

            panels.update();
        },
        (data.clone(), panels.clone(), props.index),
    );

    let toggle_pixelated = use_callback(
        move |(), (data, panels, index)| {
            let value = data.borrow().items[*index].pixelated;
            data.borrow_mut().items[*index].pixelated = !value;
            data.store();

            panels.update();
        },
        (data.clone(), panels.clone(), props.index),
    );

    let mut categories = Vec::with_capacity(item.categories.len() + 1);
    for (i, category) in item.categories.iter().enumerate() {
        let data = data.clone();
        let panels = panels.clone();
        let item_index = props.index;

        let oninput = Callback::from(move |event: InputEvent| {
            let target: HtmlInputElement = event.target().unwrap().unchecked_into();
            let value = target.value();

            if value.is_empty() {
                data.borrow_mut().items[item_index].categories.remove(i);
            } else {
                data.borrow_mut().items[item_index].categories[i] = value;
            }

            data.store();

            panels.update();
        });

        categories.push(category_input(oninput, category));
    }

    let new_category = use_callback(
        move |event: KeyboardEvent, (data, panels, index)| {
            let target: HtmlInputElement = event.target().unwrap().unchecked_into();
            let value = target.value();

            if !value.is_empty() && event.key() == "Enter" {
                data.borrow_mut().items[*index].categories.push(value);
                data.store();

                panels.update();
            }

            AnimationFrame::new(move || {
                target.focus().unwrap();
            })
            .request();
        },
        (data.clone(), panels.clone(), props.index),
    );
    let new_category = self::new_category(new_category, "");

    let input_description = use_callback(
        move |value: InputEvent, (data, panels, index)| {
            let target = value
                .target()
                .unwrap()
                .dyn_into::<HtmlTextAreaElement>()
                .unwrap();

            data.borrow_mut().items[*index].description = target.value();
            data.store();

            panels.update();
        },
        (data.clone(), panels.clone(), props.index),
    );

    let remove_item = use_callback(
        move |_, (data, panels, index)| {
            let item = data.borrow_mut().items.remove(*index);
            data.borrow_mut().removed_items.push(item);
            data.store();

            panels.right.set(RightPanel::Empty);
        },
        (data.clone(), panels.clone(), props.index),
    );

    html! {
        <div class="panel-content">
            <h2>{ "Edit Item" }</h2>

            <Spacer size=32.0/>

            <ItemView item={ item.clone() } />

            <Spacer size=32.0 />

            { text_input(input_name, "Name", &item.name) }
            { text_input(input_id, "Id", item.id.as_deref().unwrap_or("")) }
            { text_input(input_image, "Image", item.get_image()) }

            <div class="panel-row">
                <input
                    type="checkbox"
                    style="cursor: pointer;"
                    checked={ item.pixelated }
                    oninput={ toggle_pixelated.reform(|_| ()) }
                    name="pixelated"
                    title="Pixelated"
                />
                <Spacer size=10.0 direction={ Direction::Row } />
                <label
                    for="pixelated"
                    style="cursor: pointer;"
                    onclick={ toggle_pixelated.reform(|_| ()) }
                >
                    { "Pixelated" }
                </label>
            </div>

            { "Categories" }
            { for categories }
            { new_category }

            <Spacer size=32.0 />

            <textarea
                style="width: 100%; height: 100%;"
                placeholder="Description"
                title="Description"
                oninput={ input_description }
                value={ item.description }
            />

            <Spacer size=32.0 />

            <button onclick={ remove_item }>{ "Remove" }</button>
        </div>
    }
}
