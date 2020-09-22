use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
    KeyboardEvent,
    Callback,
};

use crate::routes::router;
use crate::components::nav::RouterNav;

pub struct App {
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <main id="app">
                <img src="/assets/yew-logo.svg" class="yew-logo" />
                <RouterNav />
                { router() }
            </main>
        }
    }
}

