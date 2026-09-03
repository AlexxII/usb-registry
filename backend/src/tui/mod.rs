mod app;
mod events;
mod pages;
mod ui;
mod widgets;

use std::io::Result;

use sqlx::SqlitePool;

use crate::tui::app::App;

pub fn run_tui(pool: SqlitePool) -> Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}

