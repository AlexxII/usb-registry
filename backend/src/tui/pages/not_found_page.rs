use crossterm::event::Event;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::errors::AppError;

pub struct ErrorPage {
    error: Option<AppError>,
}

impl ErrorPage {
    pub fn new(error: Option<AppError>) -> Self {
        Self { error }
    }

    pub fn render_page(&self, area: Rect, frame: &mut Frame) {
        let popup_area = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(10), // Высота карточки ошибки
            Constraint::Fill(1),
        ])
        .areas::<3>(area)[1];

        let popup_area = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(50), // Ширина карточки ошибки
            Constraint::Fill(1),
        ])
        .areas::<3>(popup_area)[1];

        let block = Block::default()
            .title(" Error ")
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .bg(Color::Indexed(234)); // Темно-серый задний фон 

        if let Some(ref err) = self.error {
            let error_text = match err {
                AppError::Validation(msg) => msg.clone(),
                AppError::BadRequest(msg) => msg.clone(),
                AppError::NotFound => "Ресурс не найден".to_string(),
                AppError::Device(dev_err) => dev_err.to_string(), // Использует макрос #[error(...)]
                AppError::Database(db_err) => format!("Ошибка БД: {:?}", db_err),
                AppError::BatchValidation(vec) => format!("Ошибок валидации пакета: {}", vec.len()),
            };

            let text = vec![
                Line::from(""), // Отступ сверху
                Line::from(Span::styled(
                    "Ошибка пиложения",
                    Style::default()
                        .fg(Color::LightRed)
                        .add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(Span::styled(error_text, Style::default().fg(Color::Gray))),
                Line::from(""),
                Line::from(Span::styled(
                    "Для выхода нажмите [q] ",
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::ITALIC),
                )),
            ];

            // Создаем виджет параграфа с центрированием текста
            let paragraph = Paragraph::new(text)
                .block(block)
                .alignment(Alignment::Center);

            frame.render_widget(paragraph, popup_area);
        }
    }

    pub fn handle_events(&mut self, _event: &Event) -> bool {
        false
    }
}
