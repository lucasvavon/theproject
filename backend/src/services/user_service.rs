use sqlx::PgPool;
use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;

pub struct UserService;

impl UserService {
    pub async fn get_user_by_id(pool: &PgPool, id: i32) -> Result<Option<User>, String> {
        UserRepository::find_by_id(pool, id).await.map_err(|e| e.to_string())
    }

    pub async fn get_users(pool: &PgPool) -> Result<Vec<User>, String> {
        UserRepository::get_all(pool).await.map_err(|e| e.to_string())
    }

    pub async fn create_user(pool: &PgPool, user: User) -> Result<u64, String> {
        UserRepository::create(pool, user).await.map_err(|e| e.to_string())
    }

    pub async fn delete_user(pool: &PgPool, id: i32) -> Result<u64, String> {
        UserRepository::delete(pool, id).await.map_err(|e| e.to_string())
    }

    /*pub fn update_user(user: User) -> Result<(), String> {
        UserRepository::update(user)
    }*/
}
