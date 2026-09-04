mod app;
mod events;
mod pages;
mod ui;
mod widgets;

use std::io::Result;

use sqlx::SqlitePool;

use crate::tui::app::App;

pub async fn run_tui(pool: SqlitePool) -> Result<()> {
    let mut app = App::new(pool).await;
    ratatui::run(|terminal| {app.run(terminal)})
}
