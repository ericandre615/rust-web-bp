use yew::prelude::{
    Component,
    ComponentLink,
    Html,
    html,
    ShouldRender,
};

use crate::routes::router;
use crate::components::nav::RouterNav;

pub struct App {
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
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

