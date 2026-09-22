use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::text::Line;
use ratatui::widgets::{HighlightSpacing, List, ListItem, ListState, StatefulWidget};

use crate::models::device::MappedDevice;

#[derive(Debug)]
pub struct DeviceList {
    pub items: Vec<MappedDevice>,
    pub state: ListState,
}

const NORMAL_ROW_BG: Color = SLATE.c950;
const TEXT_FG_COLOR: Color = SLATE.c200;

impl DeviceList {
    pub fn new(devices: Vec<MappedDevice>) -> Self {
        let items = devices;

        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }

        Self {
            items: items,
            state,
        }
    }

    pub fn render_list(&mut self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self
            .items
            .iter()
            .map(|device| ListItem::from(device))
            .collect();
        let list = List::new(items)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, area, buf, &mut self.state);
    }

    pub fn select_next(&mut self) {
        match self.state.selected() {
            None => {
                self.state.select_next();
            }
            Some(index) => {
                if index != self.items.len() - 1 {
                    self.state.select_next();
                }
            }
        }
    }

    pub fn get_selected(&self) -> Option<&MappedDevice> {
        self.state.selected().and_then(|idx| self.items.get(idx))
    }

    pub fn select_previous(&mut self) {
        if self.state.selected().is_some() {
            self.state.select_previous();
        }
    }

    pub fn select_first(&mut self) {
        self.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.state.select_last();
    }
}

impl From<&MappedDevice> for ListItem<'_> {
    fn from(value: &MappedDevice) -> Self {
        let line = match &value.manufacturer {
            Some(man) => Line::styled(format!("{}", man.clone()), TEXT_FG_COLOR),
            None => Line::styled("UNKNOWN".to_string(), TEXT_FG_COLOR),
        };
        ListItem::new(line)
    }
}
