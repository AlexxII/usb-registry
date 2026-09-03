use std::env;
use std::process::Command;

use sqlx::SqlitePool;
use tokio::net::TcpListener;

mod api;
mod db;
mod errors;
mod models;
mod os;
mod server;
mod tui;
mod usb;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

const ADDR: &str = "127.0.0.1:5151";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let db_path = os::database_path();
    let pool = db::connect(&db_path).await?;

    db::migrate(&pool).await?;

    api::auth::ensure_default_admin(&pool)
        .await
        .expect("Не удалось проверить/создать дефолтного админа");

    let mode = env::args().nth(1).unwrap_or_else(|| "tui".to_string());

    match mode.as_str() {
        "server" => run_server(pool).await?,
        "tui" => tui::run_tui(pool)?,
        _ => {
            eprintln!("Неизвестный режим: {mode}");
            eprintln!("Использование: usb-register [server|tui]");
        }
    }
    Ok(())
}

async fn run_server(pool: SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState { pool };

    let app = server::main_router(state);

    let listener = TcpListener::bind(ADDR).await?;

    clear_screen();
    println!(
        "Сервер стартанул на {ADDR}. Используйте браузеры Chrome, Safari, Mozilla для доступа. 'CTRL+C' для выхода."
    );

    axum::serve(listener, app).await?;

    Ok(())
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}
