use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph, Widget};

use super::device_list::Device;

pub struct DeviceInfo;

impl DeviceInfo {
    pub fn render(device: Option<&Device>, area: Rect, buf: &mut Buffer) {
        // Если ничего не выбрано, выводим заглушку
        let Some(dev) = device else {
            let placeholder = Paragraph::new("Выберите устройство для просмотра деталей...")
                .alignment(ratatui::layout::Alignment::Center);
            placeholder.render(area, buf);
            return;
        };

        // Формируем текст с детальной информацией
        let text = vec![
            Line::from(format!(
                "Производитель: {}",
                dev.manufacturer.as_deref().unwrap_or("Неизвестно")
            )),
            Line::from(format!(
                "Серийный номер: {}",
                dev.serial.as_deref().unwrap_or("-")
            )),
            Line::from(format!(
                "Файловая система: {}",
                dev.filesystem.as_deref().unwrap_or("-")
            )),
            Line::from(format!("Объем: {}", dev.capacity.as_deref().unwrap_or("-"))),
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Детальная информация ");

        let paragraph = Paragraph::new(text).block(block);
        paragraph.render(area, buf);
    }
}
