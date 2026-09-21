use crossterm::event::{Event, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::palette::tailwind::SLATE;
use ratatui::style::{Color, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Widget};
use tokio::sync::oneshot;

use crate::errors::{AppError, AppResult};
use crate::font::create_big_text;
use crate::models::device::MappedDevice;
use crate::tui::app::PageState;
use crate::tui::widgets::device_info::DeviceInfo;
use crate::tui::widgets::device_list::DeviceList;
use crate::tui::widgets::error::ErrorWidget;
use crate::usb::current::get_current_usb_mapped;

pub struct ConnectedPage {
    device_list: Option<DeviceList>,
    state: PageState,
    rx: Option<oneshot::Receiver<AppResult<Vec<MappedDevice>>>>,
}

impl ConnectedPage {
    const TEXT_COLOR: Color = SLATE.c400;

    pub fn new() -> Self {
        Self {
            state: PageState::Loading,
            device_list: None,
            rx: None,
        }
    }

    pub fn load(&mut self, pool: sqlx::SqlitePool) {
        let (tx, rx) = oneshot::channel();

        self.rx = Some(rx);
        self.state = PageState::Loading;

        tokio::spawn(async move {
            let result = get_current_usb_mapped(&pool).await;
            let _ = tx.send(result);
        });
    }

    pub fn poll(&mut self) {
        let Some(mut rx) = self.rx.take() else {
            return;
        };

        match rx.try_recv() {
            Ok(Ok(devices)) => {
                if !devices.is_empty() {
                    self.device_list = Some(DeviceList::new(devices));
                    self.state = PageState::Loaded;
                } else {
                    self.state = PageState::Error(AppError::BadRequest(
                        "Не удалось загрузить промапленные носители!".to_string(),
                    ))
                }
            }

            Ok(Err(error)) => {
                self.state = PageState::Error(error);
            }

            Err(oneshot::error::TryRecvError::Empty) => {
                self.rx = Some(rx);
            }

            Err(oneshot::error::TryRecvError::Closed) => {
                self.state =
                    PageState::Error(AppError::BadRequest("Загрузка устройств прервана".into()));
            }
        }
    }

    pub fn render_page(&mut self, area: Rect, frame: &mut Frame) {
        match &self.state {
            PageState::Loading => {
                self.render_loading(area, frame);
            }
            PageState::Loaded => {
                self.render_loaded(area, frame);
            }
            PageState::Error(error) => ErrorWidget::render(area, frame, error),
        }
    }

    fn render_loading(&self, area: Rect, frame: &mut Frame) {
        let [content_layout] = Layout::vertical([Constraint::Length(1)]).areas(area);
        let content = Line::from("ЗАГРУЗКА...").centered();
        Widget::render(content, content_layout, frame.buffer_mut());
    }

    fn render_loaded(&mut self, area: Rect, frame: &mut Frame) {
        if let Some(ref mut device_list) = self.device_list {

            let page_text = create_big_text("АКТИВНЫЕ", Color::Cyan);
            let page_title = Paragraph::new(page_text).alignment(Alignment::Center);

            let description = Paragraph::new("Подключенные в данный момент устройства")
                .fg(Self::TEXT_COLOR)
                .centered();

            let [title_layout, desc_layout, content_layout] = Layout::vertical([
                Constraint::Length(6),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .areas(area);

            let [list_area, details_area] =
                Layout::horizontal([Constraint::Percentage(33), Constraint::Percentage(67)])
                    .areas(content_layout);

            Widget::render(page_title, title_layout, frame.buffer_mut());
            Widget::render(description, desc_layout, frame.buffer_mut());

            device_list.render_list(list_area, frame.buffer_mut());

            let selected_device = device_list.get_selected();
            DeviceInfo::render(selected_device, details_area, frame.buffer_mut());
        }
    }

    pub fn handle_events(&mut self, event: &Event) -> bool {
        let Event::Key(key) = event else {
            return false;
        };

        if !key.is_press() {
            return false;
        }

        let Some(device_list) = &mut self.device_list else {
            return false;
        };

        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                device_list.select_next();
                return true;
            }
            KeyCode::Up | KeyCode::Char('k') => {
                device_list.select_previous();
                return true;
            }
            KeyCode::Char('G') => {
                device_list.select_last();
                return true;
            }
            KeyCode::Char('g') => {
                device_list.select_first();
                return true;
            }
            _ => false,
        }
    }
}
