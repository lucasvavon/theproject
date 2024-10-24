use sqlx::PgPool;
use crate::models::user::User;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
        User,
        "SELECT id, username, email, password, created_at, updated_at
         FROM users WHERE id = $1",
        id
    )
            .fetch_optional(pool)  // Use fetch_optional to get an Option<User>
            .await?;  // Await the future to get the actual result

        Ok(user)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(
        User,
        "SELECT * FROM users"
    )
            .fetch_all(pool) // Use fetch_all to get all users
            .await?; // Await the future to get the actual result

        Ok(users) // Return the list of users
    }

    pub async fn create(pool: &PgPool, user: User) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "INSERT INTO users (username, email, password, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)",
            user.username,
            user.email,
            user.password,
            user.created_at,
            user.updated_at
        )
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn delete(pool: &PgPool, id: i32) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        id
    )
            .execute(pool)  // Use execute for non-returning queries
            .await?;

        Ok(result.rows_affected())  // Return the number of rows affected
    }

    /*pub fn update(client: &mut Client, id: i32) -> Result<(), String> {
        client
            .execute("UPDATE users SET WHERE id = $1", &[&id])
            .map_err(|e| e.to_string())?;

        Ok(())
    }*/
}
