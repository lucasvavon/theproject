use actix_web::web;
use crate::controllers::user_controller::{create_user_handler, get_all_users_handler, get_user_handler};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("", web::post().to(create_user_handler))
            .route("", web::get().to(get_all_users_handler))
            .route("/{id}", web::get().to(get_user_handler)),
    );
}
