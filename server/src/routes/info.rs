use actix_web::{
    web,
    HttpResponse,
    Error,
    http::StatusCode,
    Resource,
};

use shared::models::api::ApiInfo;

async fn get_info() -> Result<HttpResponse, Error> {
    let api_info = ApiInfo {
        version: 1.2,
        server: "actix-web".to_string(),
        deps: vec![
            "actix-multipart 1.3".to_string(),
            "actix-web 3".to_string(),
            "actix-cors 0.3.1".to_string(),
            "futures 0.3.1".to_string(),
            "serde 1.0.115".to_string(),
            "serde_json 1.0.58".to_string(),
            "dotenv 0.15.0".to_string(),
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
