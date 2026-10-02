use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Paragraph, Widget, Wrap};

pub struct DeviceSearch;

impl DeviceSearch {
    const TEXT_COLOR: Color = SLATE.c400;

    pub fn render(area: Rect, buffer: &mut Buffer) {
        let text = "ПОИСК";
        Paragraph::new(text)
            .fg(Self::TEXT_COLOR)
            .centered()
            .wrap(Wrap { trim: false })
            .render(area, buffer);
    }
}
