use yew_router::{Switch, router::Router};
use yew::prelude::{Html, html};

use crate::views::{
    index::Index,
    info::Info,
};

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/info"]
    Info,
    #[to = "/"]
    Index,
}

pub fn router() -> Html {
    html! {
        <Router<AppRoute, ()>
            render = Router::render(|route: AppRoute| {
                match route {
                    AppRoute::Info => html!{ <Info /> },
                    AppRoute::Index => html!{ <Index /> },
                }
            })
        />
    }
}
