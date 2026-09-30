mod app;
mod error_app;
mod events;
mod pages;
mod ui;
mod widgets;

use std::io::Result;

use sqlx::SqlitePool;

use crate::tui::app::App;

use self::error_app::ErrorApp;

pub async fn run_tui(pool: SqlitePool) -> Result<()> {
    let mut app = App::new(pool);
    ratatui::run(|terminal| app.run(terminal))
}

pub async fn run_tui_error(e: sqlx::Error) -> Result<()> {
    let mut app = ErrorApp::new(e);
    ratatui::run(|terminal| app.run(terminal))
}
