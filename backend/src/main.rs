use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
mod db;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Initialize database connection
    let db_pool = db::init_pool();

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(cors)
            .configure(routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
