use std::thread;
use std::time::Duration;

use sqlx::SqlitePool;

use crate::errors::AppResult;

pub async fn check_health(pool: &SqlitePool) -> AppResult<()> {
    // thread::sleep(Duration::from_secs(4));
    sqlx::query("SELECT 1").fetch_one(pool).await?;
    Ok(())
}
