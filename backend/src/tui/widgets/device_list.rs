use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::text::Line;
use ratatui::widgets::{HighlightSpacing, List, ListItem, ListState, StatefulWidget};

use crate::models::device::MappedDevice;

#[derive(Debug)]
pub struct DeviceList {
    pub items: Vec<Device>,
    pub state: ListState,
}

#[derive(Debug)]
pub struct Device {
    id: Option<i64>,
    pub manufacturer: Option<String>,
    pub serial: Option<String>,
    pub filesystem: Option<String>,
    pub capacity: Option<String>,
}

const NORMAL_ROW_BG: Color = SLATE.c950;
const TEXT_FG_COLOR: Color = SLATE.c200;

impl DeviceList {
    pub fn new(devices: Vec<MappedDevice>) -> Self {
        let items: Vec<Device> = devices
            .into_iter()
            .map(|device| {
                Device::new(
                    device.id,
                    device.manufacturer,
                    device.serial,
                    device.filesystem,
                    device.capacity,
                )
            })
            .collect();

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

    pub fn get_selected(&self) -> Option<&Device> {
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

impl Device {
    fn new(
        id: Option<i64>,
        man: Option<String>,
        sn: Option<String>,
        fs: Option<String>,
        cap: Option<String>,
    ) -> Self {
        Self {
            id,
            manufacturer: man,
            serial: sn,
            filesystem: fs,
            // capacity: Some(format!("{} GB", cap)),
            capacity: cap
        }
    }
}

impl From<&Device> for ListItem<'_> {
    fn from(value: &Device) -> Self {
        let line = match &value.manufacturer {
            Some(man) => Line::styled(format!("{}", man.clone()), TEXT_FG_COLOR),
            None => Line::styled("UNKNOWN".to_string(), TEXT_FG_COLOR),
        };
        ListItem::new(line)
    }
}
