use crossterm::event::{Event, KeyCode};

use crate::tui::app::Page;

pub enum AppEvent {
    Quit,
    ChangePage(Page)
}

pub fn map_event(event: Event) -> Option<AppEvent> {
    match event {
        Event::Key(key_event) if key_event.is_press() => match key_event.code {
            KeyCode::Char('q') => Some(AppEvent::Quit),
            KeyCode::Char('1') => Some(AppEvent::ChangePage(Page::ConnectedPage)),
            KeyCode::Char('2') => Some(AppEvent::ChangePage(Page::HistoryPage)),
            _ => None
        }
        _ => None
    }
}
