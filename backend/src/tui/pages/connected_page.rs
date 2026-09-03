use crossterm::event::{Event, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::palette::tailwind::SLATE;
use ratatui::style::{Color, Stylize};
use ratatui::widgets::{Paragraph, Widget};
use tui_big_text::{BigText, PixelSize};

use crate::tui::widgets::device_info::DeviceInfo;
use crate::tui::widgets::device_list::DeviceList;

pub struct ConnectedPage {
    device_list: DeviceList,
}

impl ConnectedPage {
    const TEXT_COLOR: Color = SLATE.c400;

    pub fn new() -> Self {
        Self {
            device_list: DeviceList::new(),
        }
    }

    pub fn render_page(&mut self, area: Rect, frame: &mut Frame) {
        let page_title = BigText::builder()
            .pixel_size(PixelSize::ThirdHeight)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan))
            .lines(vec!["CONNECTED".into()])
            .centered()
            .build();

        let description = Paragraph::new("Подключенные в данный момент устройства")
            .fg(Self::TEXT_COLOR)
            .centered();

        let [title_layout, desc_layout, content_layout] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .areas(area);

        let [list_area, details_area] =
            Layout::horizontal([Constraint::Percentage(33), Constraint::Percentage(67)])
                .areas(content_layout);

        Widget::render(page_title, title_layout, frame.buffer_mut());
        Widget::render(description, desc_layout, frame.buffer_mut());
        self.device_list.render_list(list_area, frame.buffer_mut());

        let selected_device = self.device_list.get_selected();
        DeviceInfo::render(selected_device, details_area, frame.buffer_mut());
    }

    pub fn handle_events(&mut self, event: &Event) -> bool {
        let Event::Key(key) = event else {
            return false;
        };

        if !key.is_press() {
            return false;
        }

        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                self.device_list.select_next();
                return true;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.device_list.select_previous();
                return true;
            }
            KeyCode::Char('G') => {
                self.device_list.select_last();
                return true;
            }
            KeyCode::Char('g') => {
                self.device_list.select_first();
                return true;
            }
            _ => false,
        }
    }
}
