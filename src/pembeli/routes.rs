use actix_web::web;
use crate::pembeli::handlers::{get_buyers,get_buyer, create_buyer, update_buyer, delete_buyer};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/pembeli")
        .route("", web::get().to(get_buyers))
        .route("", web::post().to(create_buyer))
        .service(
            web::resource("/{user_id}")
            .route(web::get().to(get_buyer))
            .route(web::put().to(update_buyer))
            .route(web::delete().to(delete_buyer))
        )
    );
}
