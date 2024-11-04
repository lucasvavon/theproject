use actix_web::{App, HttpServer, web};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
use sqlx::PgPool;

mod controllers;
mod models;
mod services;
mod repositories;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");
    println!("Using DATABASE_URL: {}", database_url);
    // new pool
    let pool: PgPool = PgPool::connect(&database_url).await.expect("Failed to create pool.");

    //sqlx::migrate!("./../migrations").run(&pool).await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // share pool to all handlers
            .wrap(Cors::default()
                .allow_any_origin() /********** URL app domain to add **********/
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_header(vec![http::header::AUTHORIZATION, http::header::CONTENT_TYPE]))
            .configure(routes::user_routes::user_routes)
    })
        .bind("0.0.0.0:3030")?
        .run()
        .await
}
