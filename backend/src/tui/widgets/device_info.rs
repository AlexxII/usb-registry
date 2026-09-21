use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Widget};

use crate::models::device::MappedDevice;

pub struct DeviceInfo;

impl DeviceInfo {
    pub fn render(device: Option<&MappedDevice>, area: Rect, buf: &mut Buffer) {
        let Some(dev) = device else {
            let placeholder = Paragraph::new("Выберите устройство для просмотра деталей...")
                .alignment(ratatui::layout::Alignment::Center);
            placeholder.render(area, buf);
            return;
        };

        // Формируем текст с детальной информацией
        let text = device_info(&dev);

        let block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(1, 0, 1, 0))
            .title(" Детальная информация ");

        let paragraph = Paragraph::new(text).block(block);
        paragraph.render(area, buf);
    }
}

fn device_info(dev: &MappedDevice) -> Vec<Line<'static>> {
    let register = is_register(dev);
    let mut lines = vec![
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
        register,
        Line::from(format!(
            "Рег.№: {}",
            dev.register_number.as_deref().unwrap_or("-")
        )),
        Line::from(format!("Владелец: {}", dev.owner.as_deref().unwrap_or("-"))),
        Line::from(format!(
            "Заключение о СП: {}",
            dev.conclusion_number.as_deref().unwrap_or("-")
        )),
        Line::from(format!(
            "Предписание: {}",
            dev.prescription.as_deref().unwrap_or("-")
        )),
        Line::from(format!(
            "Гриф секретности: {}",
            dev.secclass.as_deref().unwrap_or("-")
        )),
    ];

    lines
}

fn is_register(dev: &MappedDevice) -> Line<'static> {
    if dev.registered {
        return Line::from("ЗАРЕГИСТРИРОВАН").style(Color::Green).bold();
    } else {
        return Line::from("УСТРОЙСТВО не ЗАРЕГИСТРИРОВАНО!")
            .style(Color::Red)
            .bold();
    }
}

// fn is_secret(dev: &MappedDevice) -> Line<'static> {
//     if dev.secret {}
// }
