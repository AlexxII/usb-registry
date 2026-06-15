use sqlx::SqlitePool;

pub async fn check_health(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").fetch_one(pool).await?;
    Ok(())
}
