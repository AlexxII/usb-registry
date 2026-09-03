use crossterm::event::Event;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

pub struct NotFoundPage;

impl NotFoundPage {
    pub fn new() -> Self {
        Self
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

        // Текст внутри карточки
        let text = vec![
            Line::from(""), // Отступ сверху
            Line::from(Span::styled(
                "500 - DataBase NOT FOUND",
                Style::default()
                    .fg(Color::LightRed)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "База данных не найдена",
                Style::default().fg(Color::Gray),
            )),
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

    pub fn handle_events(&mut self, _event: &Event) -> bool {
        false
    }
}
