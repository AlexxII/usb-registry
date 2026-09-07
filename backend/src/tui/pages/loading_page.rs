use crossterm::event::Event;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::Line;
use ratatui::widgets::Widget;

use crate::errors::{AppError, AppResult};

pub struct LoadingPage {
    rx: tokio::sync::oneshot::Receiver<AppResult<()>>,
}

impl LoadingPage {
    pub fn new(rx: tokio::sync::oneshot::Receiver<AppResult<()>>) -> Self {
        Self { rx }
    }

    pub fn try_recv(&mut self) -> Option<AppResult<()>> {
        match self.rx.try_recv() {
            Ok(result) => Some(result), // Загрузка завершена успешно или с ошибкой
            Err(tokio::sync::oneshot::error::TryRecvError::Empty) => None, // Всё еще грузится
            Err(tokio::sync::oneshot::error::TryRecvError::Closed) => {
                // Фоновое задание умерло по какой-то причине
                Some(Err(AppError::BadRequest(
                    "Поток загрузки оборвался".to_string(),
                )))
            }
        }
    }

    pub fn render_page(&self, area: Rect, frame: &mut Frame) {
        let page_title = Line::from("Loading Page").centered();

        let [title_layout, content_layout] =
            Layout::vertical([Constraint::Length(1), Constraint::Percentage(90)]).areas(area);

        let content = Line::from("LOADING...").centered();

        Widget::render(page_title, title_layout, frame.buffer_mut());
        Widget::render(content, content_layout, frame.buffer_mut());
    }

    pub fn handle_events(&mut self, event: &Event) -> bool {
        false
    }
}
