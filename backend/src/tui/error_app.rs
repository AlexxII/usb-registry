use std::io::Result;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Padding};
use ratatui::{DefaultTerminal, Frame};

use crate::errors::AppError;

use super::widgets::error::ErrorWidget;

pub struct ErrorApp {
    exit: bool,
    error: AppError,
}

impl ErrorApp {
    pub fn new(err: sqlx::Error) -> Self {
        Self {
            exit: false,
            error: AppError::Database(err),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame))?;

            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char('q') => self.exit = true,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let block = Block::bordered()
            .padding(Padding::uniform(1))
            .title_top(Line::from("USB-registry").centered())
            .border_style(Style::new().fg(Color::Yellow))
            .border_type(BorderType::Rounded);

        let inner_area = block.inner(frame.area());

        ErrorWidget::render(inner_area, frame, &self.error);

        frame.render_widget(block, frame.area());
    }
}
