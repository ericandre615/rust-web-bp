mod info;

use actix_web::web::ServiceConfig;

pub fn router(cfg: &mut ServiceConfig) {
    cfg
        .service(info::info_routes());
}
