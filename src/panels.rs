use garlic::{AnimationFrame, Style};
use yew::prelude::*;

use crate::{
    main_panel::{MainPanel, MainPanelView},
    right_panel::{RightPanel, RightPanelView},
};

#[derive(Clone, Debug, PartialEq)]
pub struct PanelContext {
    pub main: UseStateHandle<MainPanel>,
    pub right: UseStateHandle<RightPanel>,
    update: Callback<()>,
}

impl PanelContext {
    pub fn update(&self) {
        self.update.emit(());
    }

    pub fn request_animation_frame(&self) {
        let update = self.update.clone();
        AnimationFrame::new(move || update.emit(())).request();
    }
}

#[function_component]
pub fn Panels() -> Html {
    let main = use_state(|| MainPanel::Inventory);
    let right = use_state(|| RightPanel::Empty);

    let force_update = use_force_update();

    let context = PanelContext {
        main: main.clone(),
        right: right.clone(),
        update: Callback::from(move |_| force_update.force_update()),
    };

    html! {
        <ContextProvider<PanelContext> context={ context }>
            <div class="panel-menu">
                { panel_button(MainPanel::Inventory, &main) }
                { panel_button(MainPanel::Items, &main) }
            </div>

            <div class="panels">
                <MainPanelView />
                <RightPanelView />
            </div>
        </ContextProvider<PanelContext>>
    }
}

fn panel_button(panel: MainPanel, handle: &UseStateHandle<MainPanel>) -> Html {
    let panel_name = match panel {
        MainPanel::Inventory => "Inventory",
        MainPanel::Items => "Items",
    };

    let mut style = Style::default();

    if **handle == panel {
        style.set("color", "var(--text-color-light)");
        style.set("background-color", "var(--primary-color-dark)");
    }

    let handle = handle.clone();
    html! {
        <button
            class="panel-button"
            style={ style }
            onclick={ Callback::from(move |_| handle.set(panel)) }
        >
            { panel_name }
        </button>
    }
}
