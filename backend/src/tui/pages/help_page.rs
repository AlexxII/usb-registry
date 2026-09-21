use crossterm::event::Event;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize, palette::tailwind::SLATE},
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
};
use crate::font::create_big_text;

pub struct HelpPage;

impl HelpPage {
    const TEXT_COLOR: Color = SLATE.c400;

    pub fn new() -> Self {
        Self
    }

    pub fn render_page(&self, area: Rect, frame: &mut Frame) {
        let page_text = create_big_text("ПОМОЩЬ", Color::Cyan);
        let page_title = Paragraph::new(page_text).alignment(Alignment::Center);

        let description = Paragraph::new("ПОМОЩЬ! Прочтите внимательно!")
            .fg(Self::TEXT_COLOR)
            .centered();

        let [title_layout, desc_layout, content_layout] = Layout::vertical([
            Constraint::Length(6),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .areas(area);

        let text = vec![
            Line::from(""), // Пустая строка для отступа сверху
            Line::from(vec![Span::raw(
                "Для сыночек-корзиночек и девочек-припевочек предусмотрен WEB-интерфейс. ",
            )]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Чтобы запустить приложение в качестве сервера - используйте аргумент: "),
                Span::styled(
                    "server",
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw("."),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Пример команды: "),
                Span::styled(
                    "cargo run -- server",
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::raw(
                " Далее используйте web-браузер Chrome, Safari или Mozilla. Браузеры компании МелкоМягкие не поддерживаются, мне очень жаль.",
            )]),
            Line::from(""),
        ];

        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true }); // Автоперенос строк, если окно терминала узкое

        Widget::render(page_title, title_layout, frame.buffer_mut());
        Widget::render(description, desc_layout, frame.buffer_mut());

        frame.render_widget(paragraph, content_layout);
    }

    pub fn handle_events(&mut self, _event: &Event) -> bool {
        false
    }
}
