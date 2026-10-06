use sqlx::SqlitePool;

use crate::errors::AppResult;

pub async fn check_health(pool: &SqlitePool) -> AppResult<()> {
    sqlx::query("SELECT 1").fetch_one(pool).await?;
    Ok(())
}
