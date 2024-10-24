use serde::{Serialize, Deserialize};
use sqlx::{FromRow};
use chrono::{NaiveDateTime};

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Option<i32>,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}