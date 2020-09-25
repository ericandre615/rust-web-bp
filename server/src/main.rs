use actix_cors::Cors;
use actix_web::{
    middleware,
    web,
    App,
    HttpResponse,
    HttpServer,
};
use dotenv::dotenv;
use env_logger;

mod routes;
use routes::router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "actx_web=debug,actix_web=info,actix_web=error,actix_server=error,actix_server=info");
    env_logger::init();

    let (default_host, default_port) = (String::from("0.0.0.0"), String::from("3000"));
    let host = dotenv::var("API_HOST").unwrap_or(default_host);
    let port = dotenv::var("API_PORT").unwrap_or(default_port);

    let ip = format!("{}:{}", host, port);

    HttpServer::new(|| {
        let cors = Cors::default();

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
