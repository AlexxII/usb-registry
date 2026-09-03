use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, BorderType, Padding};

use crate::tui::app::{App, Page};
use crate::tui::widgets::help::HelpWidget;

pub struct Ui;

impl Ui {
    pub fn render(app: &mut App, frame: &mut Frame) {
        let block = Block::bordered()
            .padding(Padding::uniform(1))
            .title_top(Line::from("USB-registry").centered())
            .border_style(Style::new().fg(Color::Yellow))
            .border_type(BorderType::Rounded);

        let inner_area = block.inner(frame.area());
        let [content_area, help_area] =
            Layout::vertical([Constraint::Percentage(98), Constraint::Length(2)]).areas(inner_area);

        HelpWidget::render(help_area, frame.buffer_mut());

        match app.page {
            Page::NotFoundPage => app.not_found_page.render_page(content_area, frame),
            Page::ConnectedPage => app.connected_page.render_page(content_area, frame),
            Page::HistoryPage => app.history_page.render_page(content_area, frame),
            Page::LoadingPage => app.loading_page.render_page(content_area, frame),
        };
        frame.render_widget(block, frame.area());
    }
}
