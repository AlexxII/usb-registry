use std::io::Result;

use crossterm::event::{self, Event};
use ratatui::DefaultTerminal;
use sqlx::SqlitePool;

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
    pub fn new(pool: SqlitePool) -> Self {
        let mut connected_page = ConnectedPage::new();
        connected_page.load(pool.clone());

        let mut history_page = HistoryPage::new();
        history_page.load(pool.clone());

        Self {
            exit: false,
            connected_page: connected_page,
            history_page: history_page,
            help_page: HelpPage::new(),
            page: Page::ConnectedPage,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| Ui::render(self, frame))?;

            self.connected_page.poll();
            self.history_page.poll();

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
