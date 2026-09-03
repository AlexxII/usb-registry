use crossterm::event::Event;
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::Line;
use ratatui::widgets::{Widget};

pub struct LoadingPage;

impl LoadingPage {
    pub fn new() -> Self {
        Self
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
