use crate::{db::DbPool, models::User};
use actix_web::{web, HttpResponse};
use diesel::prelude::*;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").route(web::get().to(get_users)));
}

async fn get_users(pool: web::Data<DbPool>) -> HttpResponse {
    use crate::schema::users::dsl::*;

    let conn = pool.get().expect("Couldn't get DB connection.");
    let result = web::block(move || users.load::<User>(&conn)).await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
