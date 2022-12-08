use yew::prelude::*;

use crate::{item_editor::ItemEditor, panels::PanelContext};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RightPanel {
    Empty,
    ItemEditor { index: usize },
}

impl RightPanel {
    pub fn html(&self) -> Html {
        match *self {
            RightPanel::Empty => html! {},
            RightPanel::ItemEditor { index } => html! {
                <div class="side-panel">
                    <ItemEditor index={ index } />
                </div>
            },
        }
    }
}

#[function_component]
pub fn RightPanelView() -> Html {
    let panels = use_context::<PanelContext>().unwrap();

    panels.right.html()
}
