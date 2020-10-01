use yew::{
    Component,
    ComponentLink,
    ShouldRender,
    Html,
    html,
    format::Nothing,
};
use yew::services::{ConsoleService, FetchService};
use yew::services::fetch::{Request, Response, FetchTask};
use serde::{Serialize, Deserialize};
use dotenv_codegen::dotenv;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiInfo {
    version: f32,
    server: String,
    deps: Vec<String>,
}

pub enum Msg {
    FetchInfo,
    FetchComplete(ApiInfo),
    FetchError(String),
}

#[derive(Debug)]
pub struct Info {
    link: ComponentLink<Self>,
    info: Option<ApiInfo>,
    fetch_task: Option<FetchTask>,
}

impl Component for Info {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            info: None,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Msg) -> ShouldRender {
        match msg {
            Msg::FetchComplete(res) => {
                ConsoleService::info(&format!("Fetched {:?}", res));
                self.info = Some(res);
            },
            Msg::FetchError(err) => {
                ConsoleService::info(&format!("FetchErr {:?}", err));
                self.info = None;
            },
            Msg::FetchInfo => {
                let host = dotenv!("API_HOST"); //dotenv::var("API_HOST").unwrap();
                let port = dotenv!("API_PORT"); //dotenv::var("API_PORT").unwrap();
                let api_uri = format!("http://{}:{}", host, port);
                let get_request = Request::get(format!("{}/info", api_uri))
                    .header("Accept", "application/json")
                    .header("Content-Type", "application/json")
                    .body(Nothing)
                    .expect("Failed to get resource");
                let task = FetchService::fetch(
                    get_request,
                    self.link.callback(|res: Response<Result<String, _>>| {
                        if res.status().is_success() {
                            ConsoleService::info("Fetch Success");
                            let body = res.body().as_ref().unwrap();
                            let api_info: ApiInfo = serde_json::from_str(&body).unwrap();
                            Msg::FetchComplete(api_info)
                        } else {
                            ConsoleService::info("Fetch Error");
                            Msg::FetchError(String::from("Fetch Failed"))
                        }
                    })
                ).unwrap();

                self.fetch_task = Some(task);
            },
        }

        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div id="info-view" style="width:100%;">
                <h1>{ "System Info" }</h1>
                <button
                    type="button"
                    onclick=self.link.callback(|_| Msg::FetchInfo)
                >
                    { "Get Info" }
                </button>
                { self.render_info() }
            </div>
        }
    }
}

impl Info {
    fn render_info(&self) -> Html {
        let app_info = match &self.info {
            Some(info) => {
                let api_info: &ApiInfo = info;
                let version = api_info.version;
                let server = api_info.server.clone();
                html! {
                    <div class="info">
                        <p class="api-version">{ format!("version: {}",version) }</p>
                        <p class="api-server">{ format!("server: {}",server) }</p>
                        <ul class="deps-list">
                            {
                                for api_info.deps.iter().map(|dep| {
                                    html! { <li>{ dep }</li> }
                                })
                            }
                        </ul>
                    </div>
                }
            },
            None => html! { <></> }
        };

        app_info
    }
}
