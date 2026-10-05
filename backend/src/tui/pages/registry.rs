use crate::errors::{AppError, AppResult};
use crate::font_ex::create_big_text;
use crate::models::device::MappedDevice;
use crate::tui::app::PageState;
use crate::tui::widgets::device_info::DeviceInfo;
use crate::tui::widgets::device_list::DeviceList;
use crate::tui::widgets::device_search::DeviceSearch;
use crate::tui::widgets::error::ErrorWidget;
use crate::usb::registry::get_usb_from_db;
use crossterm::event::{Event, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::palette::tailwind::SLATE;
use ratatui::style::{Color, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Widget, Wrap};
use tokio::sync::oneshot;

pub struct RegistryPage {
    device_list: Option<DeviceList>,
    state: PageState,
    all_devices: Vec<MappedDevice>,
    device_search: DeviceSearch,
    rx: Option<oneshot::Receiver<AppResult<Vec<MappedDevice>>>>,
}

impl RegistryPage {
    const TEXT_COLOR: Color = SLATE.c400;

    pub fn new() -> Self {
        Self {
            state: PageState::Loading,
            device_list: None,
            all_devices: Vec::new(),
            device_search: DeviceSearch::new(),
            rx: None,
        }
    }

    pub fn load(&mut self, pool: sqlx::SqlitePool) {
        let (tx, rx) = oneshot::channel();

        self.rx = Some(rx);
        self.state = PageState::Loading;

        tokio::spawn(async move {
            let result = get_usb_from_db(&pool).await;
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
                    self.all_devices = devices.clone();
                    // self.device_list = Some(DeviceList::new(devices));
                    self.state = PageState::Loaded;
                } else {
                    self.state = PageState::Error(AppError::BadRequest(
                        "Не удалось загрузить информацию из базы данных!".to_string(),
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
        let page_text = create_big_text("РЕЕСТР", Color::Cyan);
        let page_title = Paragraph::new(page_text).alignment(Alignment::Center);

        let description = Paragraph::new("Реестр зарегистрированных устройств")
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

        let [search_area, list_area] =
            Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)])
                .areas(list_area);

        Widget::render(page_title, title_layout, frame.buffer_mut());
        Widget::render(description, desc_layout, frame.buffer_mut());
        self.device_search.render(frame, search_area);

        if let Some(ref mut device_list) = self.device_list {
            device_list.render_list(list_area, frame.buffer_mut());

            let selected_device = device_list.get_selected();

            // пользовательский поиск
            let query = match self.device_search.input.value() {
                "" => None,
                q => Some(q)
            };
            
            DeviceInfo::render(selected_device, details_area, frame.buffer_mut(), false, query);
        } else {
            let message = Paragraph::new(vec![
                Line::from("Начните ввод для поиска. Минимум 2 символа."),
                Line::from("Поиск по производителю, регистрационному номеру, серийному номеру и владельцу.")
                    .italic(),
            ])
            .centered()
            .wrap(Wrap { trim: true });

            Widget::render(message, list_area, frame.buffer_mut());
        }
    }

    pub fn handle_events(&mut self, event: &Event) -> bool {
        if let Some(query) = self.device_search.handle_event(event) {
            self.search_devices(&query);
            return true;
        }

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

    fn search_devices(&mut self, query: &str) {
        let query = query.trim();

        if query.chars().count() < 2 {
            self.device_list = None;
            return;
        }

        let query = query.to_lowercase();

        let devices = self
            .all_devices
            .iter()
            .filter(|device| {
                device
                    .manufacturer
                    .as_deref()
                    .unwrap_or("")
                    .to_lowercase()
                    .contains(&query)
                    || device
                        .serial
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query)
                    || device
                        .register_number
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query)
                    || device
                        .owner
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query)
            })
            .cloned()
            .collect();

        self.device_list = Some(DeviceList::new(devices));
    }
}
