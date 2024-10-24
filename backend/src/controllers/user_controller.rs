use actix_web::{web, HttpResponse, Responder};
use crate::services::user_service::UserService;
use crate::models::user::User;
use sqlx::PgPool;

pub async fn create_user_handler(pool: web::Data<PgPool>, user: web::Json<User>) -> impl Responder {
    match UserService::create_user(pool.get_ref(), user.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("User created successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to create user: {}", e)),
    }
}

pub async fn get_user_handler(pool: web::Data<PgPool>, user_id: web::Path<i32>) -> impl Responder {
    match UserService::get_user_by_id(pool.get_ref(), user_id.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),  // User found, return 200 with user data
        Ok(None) => HttpResponse::NotFound().body("User not found"),  // No user found, return 404
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),  // Handle DB error
    }
}

pub async fn get_all_users_handler(pool: web::Data<PgPool>) -> impl Responder {
    match UserService::get_users(pool.get_ref()).await {
        Ok(users) if !users.is_empty() => HttpResponse::Ok().json(users),  // User found, return 200 with user data
        Ok(_) => HttpResponse::NotFound().body("No users found"),  // No user found, return 404
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),  // Handle DB error
    }
}
