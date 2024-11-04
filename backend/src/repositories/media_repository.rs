use sqlx::PgPool;
use crate::models::media::{Media, Provider};

pub struct MediaRepository {
    pool: PgPool,
}

impl MediaRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_media(&self, media: &Media) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO media (id, title, provider, media_type, year, rating)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO UPDATE SET
             title = EXCLUDED.title,
             rating = EXCLUDED.rating",
            media.id,
            media.title,
            media.provider as _,
            media.media_type as _,
            media.year,
            media.rating
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn find_by_provider(&self, provider: Provider) -> Result<Vec<Media>, sqlx::Error> {
        sqlx::query_as!(
            Media,
            "SELECT * FROM media WHERE provider = $1",
            provider as _
        )
            .fetch_all(&self.pool)
            .await
    }
}