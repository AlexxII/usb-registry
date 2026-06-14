use std::str::FromStr;

use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use tokio::net::TcpListener;

mod api;
mod models;
mod server;
mod usb;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool
}

const ADDR: &str = "127.0.0.1:5151";
const DATABASE_URL: &str = "sqlite:app.db";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // TODO - read about sqlite connection options + understand what is SqliteJournalMode
    let options = SqliteConnectOptions::from_str(DATABASE_URL)?
        .create_if_missing(true)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    // TODO - check that file is there!!
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    let state = AppState {
        pool
    };

    let app = server::main_router(state);

    let listener = TcpListener::bind(ADDR)
        .await
        .expect("Failed to create a TCP listener!");

    println!("{}", format!("Server starts on {}", ADDR));

    axum::serve(listener, app)
        .await
        .expect("Failed to start a web-server!");
    Ok(())
}
