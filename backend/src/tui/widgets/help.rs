use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Paragraph, Widget, Wrap};

pub struct HelpWidget;

impl HelpWidget {
    const TEXT_COLOR: Color = SLATE.c400;

    pub fn render(area: Rect, buffer: &mut Buffer) {
        let text = "▲ ▼ | j/k: Перемещение, g/G: Начало/Конец, 1: Подключено, 2: История, q: Выход";
        Paragraph::new(text)
            .fg(Self::TEXT_COLOR)
            .centered()
            .wrap(Wrap { trim: false })
            .render(area, buffer);
    }
}
