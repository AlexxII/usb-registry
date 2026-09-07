use std::io::Result;

use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use sqlx::SqlitePool;

use crate::db::health::check_health;
use crate::errors::AppError;
use crate::tui::events::{self, AppEvent};
use crate::tui::pages::connected_page::ConnectedPage;
use crate::tui::pages::help_page::HelpPage;
use crate::tui::pages::history_page::HistoryPage;
use crate::tui::ui::Ui;

pub struct App {
    pub exit: bool,
    pub page: Page,
    pub connected_page: ConnectedPage,
    pub history_page: HistoryPage,
    pub help_page: HelpPage,
}

#[derive(PartialEq)]
pub enum Page {
    ConnectedPage,
    HistoryPage,
    HelpPage,
}

pub enum PageState {
    Loading,
    Loaded,
    Error(AppError),
}

impl App {
    pub async fn new(pool: SqlitePool) -> Self {
        let (tx, rx) = tokio::sync::oneshot::channel();

        let health_pool = pool.clone();

        tokio::spawn(async move {
            let result = check_health(&health_pool).await;
            let _ = tx.send(result);
        });
        Self {
            exit: false,
            connected_page: ConnectedPage::new(),
            history_page: HistoryPage::new(),
            help_page: HelpPage::new(),
            page: Page::ConnectedPage,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| Ui::render(self, frame))?;

            if event::poll(std::time::Duration::from_millis(50))? {
                if let Ok(event) = event::read() {
                    self.update(event);
                }
            }
        }

        Ok(())
    }

    fn update(&mut self, event: Event) {
        let handled = match self.page {
            Page::ConnectedPage => self.connected_page.handle_events(&event),
            Page::HistoryPage => self.history_page.handle_events(&event),
            Page::HelpPage => self.help_page.handle_events(&event),
            _ => false,
        };
        if !handled {
            match events::map_event(event) {
                Some(event) => match event {
                    AppEvent::Quit => self.exit(),
                    AppEvent::ChangePage(page) => match page {
                        Page::ConnectedPage => self.set_page(Page::ConnectedPage),
                        Page::HistoryPage => self.set_page(Page::HistoryPage),
                        Page::HelpPage => self.set_page(Page::HelpPage),
                    },
                },
                None => {}
            }
        }
    }
}

impl App {
    fn exit(&mut self) {
        self.exit = true;
    }

    fn set_page(&mut self, page: Page) {
        self.page = page
    }
}
