use std::{cell::RefCell, rc::Rc};

use cauldron_data::Data;
use deref_derive::{Deref, DerefMut};
use yew::{prelude::*, suspense::Suspension};

tauri_invoke::invoke! {
    pub async fn load_data() -> Data;
    pub async fn store_data(data: &Data);
}

#[derive(Clone, Debug, PartialEq, Deref, DerefMut)]
pub struct DataContext {
    #[deref]
    pub data: Rc<RefCell<Data>>,
}

impl DataContext {
    pub fn reload(&self) -> Suspension {
        let inventory = self.data.clone();

        Suspension::from_future(async move {
            *inventory.borrow_mut() = load_data().await.unwrap();
        })
    }

    pub fn store(&self) -> Suspension {
        let inventory = self.data.clone();
        let store = async move {
            let inventory = inventory.borrow().clone();
            store_data(&inventory).await.unwrap();
        };

        Suspension::from_future(store)
    }
}

impl From<Rc<RefCell<Data>>> for DataContext {
    fn from(data: Rc<RefCell<Data>>) -> Self {
        Self { data }
    }
}

#[derive(Properties, PartialEq)]
pub struct DataLoaderProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component]
pub fn DataLoader(props: &DataLoaderProps) -> Html {
    let data = use_mut_ref(Data::default);

    let is_first = use_state(|| true);
    if *is_first {
        is_first.set(false);

        let data = data.clone();
        Suspension::from_future(async move {
            *data.borrow_mut() = load_data().await.unwrap();
        });
    }

    let context = DataContext::from(data.clone());

    html! {
        <ContextProvider<DataContext> context={ context }>
            { for props.children.iter() }
        </ContextProvider<DataContext>>
    }
}
