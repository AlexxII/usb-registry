use crossterm::event::Event;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::palette::tailwind::SLATE;
use ratatui::style::{Color, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Widget};
use tui_big_text::{BigText, PixelSize};

pub struct HistoryPage;

impl HistoryPage {
    const TEXT_COLOR: Color = SLATE.c400;

    pub fn new() -> Self {
        Self
    }

    pub fn render_page(&self, area: Rect, frame: &mut Frame) {
        let page_title = BigText::builder()
            .pixel_size(PixelSize::ThirdHeight)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan))
            .lines(vec!["HISTORY".into()])
            .centered()
            .build();

        let description = Paragraph::new("История подключений")
            .fg(Self::TEXT_COLOR)
            .centered();

        let [title_layout, desc_layout, content_layout] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Percentage(90),
        ])
        .areas(area);

        let content = Line::from("CONTENT").centered();

        Widget::render(page_title, title_layout, frame.buffer_mut());
        Widget::render(description, desc_layout, frame.buffer_mut());
        Widget::render(content, content_layout, frame.buffer_mut());
    }

    pub fn handle_events(&mut self, event: &Event) -> bool {
        false
    }
}
