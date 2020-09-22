use std::io::Write;
use std::fs::File;

use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{
    middleware,
    web,
    App,
    Error,
    HttpResponse,
    HttpServer,
    http::{StatusCode}
};
use futures::{StreamExt, TryStreamExt};
use dotenv::dotenv;

mod routes;
use routes::router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actix_server=info");
    let host = dotenv::var("API_HOST").unwrap_or(String::from("0.0.0.0"));
    let port = dotenv::var("API_PORT").unwrap_or(String::from("3000"));

    let ip = format!("{}:{}", host, port);

    HttpServer::new(|| {
        let cors = Cors::default(); //.supports_credentials();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(cors)
            .configure(router)
            .default_service(web::route()
                .to(|| HttpResponse::NotFound()
                    .content_type("text/plain")
                    .body("Not Found"))
            )
    })
    .bind(ip)?
    .run()
    .await
}
