use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::style::palette::tailwind::SLATE;
use ratatui::text::Line;
use ratatui::widgets::{HighlightSpacing, List, ListItem, ListState, StatefulWidget, Widget};

pub struct DeviceList {
    pub items: Vec<Device>,
    pub state: ListState,
}

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
    pub fn new() -> Self {
        let items: Vec<Device> = [
            (1_u8, "ADATA", "1234124HJSDJ", "NTFS", 16_u8),
            (2_u8, "Kingston", "GHJER234234FB", "ext4", 32_u8),
            (3_u8, "Maxtor", "234823748", "FAT32", 4_u8),
        ]
        .into_iter()
        .map(|(id, man, sn, fs, cap)| Device::new(id, man, sn, fs, cap))
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
    fn new(id: u8, man: &str, sn: &str, fs: &str, cap: u8) -> Self {
        Self {
            id: Some(id as i64),
            manufacturer: Some(man.to_string()),
            serial: Some(sn.to_string()),
            filesystem: Some(fs.to_string()),
            capacity: Some(format!("{} GB", cap)),
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
