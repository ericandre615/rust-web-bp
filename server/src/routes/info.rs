use actix_web::{
    web,
    HttpResponse,
    Error,
    http::StatusCode,
    Resource,
};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiInfo {
    version: f32,
    server: &'static str,
    deps: Vec<&'static str>,
}

async fn get_info() -> Result<HttpResponse, Error> {
    let api_info = ApiInfo {
        version: 1.2,
        server: "actix-web",
        deps: vec![
            "actix-multipart 0.3",
            "actix-web 3",
            "actix-cors 0.3.0",
            "futures 0.3.1",
            "serde 1.0.115",
            "serde_json 1.0.57",
            "dotenv 0.15.0",
            "cargo-watch",
        ],
    };

    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("application/json; charset=utf-8")
            .json(api_info)
    )
}

pub fn info_routes() -> Resource {
    web::resource("/info")
        .route(web::get().to(get_info))
}
