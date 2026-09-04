use std::io::Result;

use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use sqlx::SqlitePool;

use crate::db::health::check_health;
use crate::tui::events::{self, AppEvent};
use crate::tui::pages::connected_page::ConnectedPage;
use crate::tui::pages::help_page::HelpPage;
use crate::tui::pages::history_page::HistoryPage;
use crate::tui::pages::loading_page::LoadingPage;
use crate::tui::pages::not_found_page::ErrorPage;
use crate::tui::ui::Ui;

pub struct App {
    pub exit: bool,
    pub page: Page,
    pub loading_page: LoadingPage,
    pub connected_page: ConnectedPage,
    pub history_page: HistoryPage,
    pub error_page: ErrorPage,
    pub help_page: HelpPage,
}

pub enum Page {
    ErrorPage,
    LoadingPage,
    ConnectedPage,
    HistoryPage,
    HelpPage,
}

impl App {
    pub async fn new(pool: SqlitePool) -> Self {
        let (error_page, initial_page) = match check_health(&pool).await {
            Ok(_) => (ErrorPage::new(None), Page::ConnectedPage),
            Err(err) => (ErrorPage::new(Some(err)), Page::ErrorPage),
        };
        Self {
            exit: false,
            loading_page: LoadingPage::new(),
            connected_page: ConnectedPage::new(),
            history_page: HistoryPage::new(),
            error_page,
            help_page: HelpPage::new(),
            page: initial_page,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| Ui::render(self, frame))?;

            if let Ok(event) = event::read() {
                self.update(event);
            }
        }
        Ok(())
    }

    fn update(&mut self, event: Event) {
        let handled = match self.page {
            Page::ErrorPage => self.error_page.handle_events(&event),
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
                        Page::ErrorPage => self.set_page(Page::ErrorPage),
                        Page::ConnectedPage => self.set_page(Page::ConnectedPage),
                        Page::HistoryPage => self.set_page(Page::HistoryPage),
                        Page::LoadingPage => self.set_page(Page::LoadingPage),
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
