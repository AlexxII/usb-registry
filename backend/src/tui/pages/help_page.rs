use crossterm::event::Event;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize, palette::tailwind::SLATE},
    text::{Line, Span},
    widgets::{Paragraph, Widget, Wrap},
};
use tui_big_text::{BigText, PixelSize};

pub struct HelpPage;

impl HelpPage {
    const TEXT_COLOR: Color = SLATE.c400;

    pub fn new() -> Self {
        Self
    }

    pub fn render_page(&self, area: Rect, frame: &mut Frame) {
        // 1. Создаем рамку для страницы помощи
        let page_title = BigText::builder()
            .pixel_size(PixelSize::ThirdHeight)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan))
            .lines(vec!["HELP".into()])
            .centered()
            .build();

        let description = Paragraph::new("ПОМОЩЬ! Прочтите внимательно!")
            .fg(Self::TEXT_COLOR)
            .centered();

        let [title_layout, desc_layout, content_layout] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .areas(area);

        // 2. Формируем текст с подсветкой важных элементов
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
