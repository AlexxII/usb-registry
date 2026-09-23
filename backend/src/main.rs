use std::env;
use std::process::Command;

use sqlx::SqlitePool;
use tokio::net::TcpListener;

mod api;
mod db;
mod errors;
mod font;
mod font_ex;
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

    let args: Vec<String> = env::args().skip(1).collect();
    let should_init = args.iter().any(|arg| arg == "--init" || arg == "-i");

    let mode = args
        .iter()
        .find(|arg| !arg.starts_with('-'))
        .map(|s| s.as_str())
        .unwrap_or("tui");

    let db_path = os::database_path();

    // 1. Подключаемся. База создается ТОЛЬКО если был передан флаг `--init`
    let pool = match db::connect(&db_path, should_init).await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Ошибка подключения к БД: {e}");
            eprintln!(
                "Возможно, база данных не создана. Используйте флаг `--init`, чтобы инициализировать приложение:"
            );
            eprintln!("  usb-register --init {mode}");
            std::process::exit(1);
        }
    };

    if should_init {
        println!("Инициализация базы данных и запуск миграций...");
        db::migrate(&pool).await?;
        api::auth::ensure_default_admin(&pool)
            .await
            .expect("Не удалось проверить/создать дефолтного админа");

        if args.len() == 1 && should_init {
            println!("База данных успешно инициализирована!");
            return Ok(());
        }
    }
    match mode {
        "server" => run_server(pool).await?,
        "tui" => tui::run_tui(pool).await?,
        _ => {
            eprintln!("Неизвестный режим: {mode}");
            eprintln!("Использование: usb-register [--init] [server|tui]");
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
