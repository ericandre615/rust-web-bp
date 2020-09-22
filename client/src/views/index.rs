use yew::{
    Component,
    ComponentLink,
    Properties,
    ShouldRender,
    Html,
    html,
};

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct IndexProps {}

pub enum Msg {}

#[derive(Debug)]
pub struct Index {
    props: IndexProps,
    link: ComponentLink<Self>,
}

impl Component for Index {
    type Message = Msg;
    type Properties = IndexProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
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
            <div id="index-view" style="width:100%;">
                <h1>{ "Rust | Yew | Actix" }</h1>
                <p>{ "Basic boilerplate setup for Rust and Yew" }</p>
            </div>
        }
    }
}
