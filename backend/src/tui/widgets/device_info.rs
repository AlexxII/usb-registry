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
    let register_info = register_info(dev);
    let special_dev = is_special(dev);
    let internet = is_internet(dev);
    let max_secclas = show_maxsecclass(dev);
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
    ];

    if dev.registered & dev.secret {
        lines.extend(register_info);
        lines.extend(special_dev);
        lines.extend(max_secclas);
        if dev.destroyed {
            lines.push(Line::from("Носитель УНИЧТОЖЕН").style(Color::Red))
        }
    } else if dev.registered & !dev.secret {
        lines.extend(internet);
    } else {
        lines.push(
            Line::from("УСТРОЙСТВО не ЗАРЕГИСТРИРОВАНО!")
                .style(Color::Red)
                .bold(),
        );
    }

    lines
}

fn register_info(dev: &MappedDevice) -> Vec<Line<'static>> {
    if dev.registered {
        return vec![
            Line::from("ЗАРЕГИСТРИРОВАН").style(Color::Green).bold(),
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
            Line::from(format!("Зоны: {}", dev.zones.as_deref().unwrap_or("-"))),
        ];
    } else {
        vec![]
    }
}

fn is_special(dev: &MappedDevice) -> Vec<Line<'static>> {
    if dev.special {
        vec![
            Line::from("СПЕЦИАЛЬНОЕ ДЕЛОПРОИЗВОДСТВО")
                .style(Color::Red)
                .bold(),
            Line::from(
                "если ваш диск не зарегистрирован на участке СПЕЦИАЛЬНОГО делопроизводства - Вы попали!",
            ),
        ]
    } else {
        vec![]
    }
}

fn is_internet(dev: &MappedDevice) -> Vec<Line<'static>> {
    if !dev.secret {
        vec![
            Line::from("ДЛЯ АП ИНТЕРНЕТ").style(Color::Red).bold(),
            Line::from("Если вы сидите за ОВТ и видите это сообщение - Вы попали!"),
        ]
    } else {
        vec![]
    }
}

fn show_maxsecclass(dev: &MappedDevice) -> Vec<Line<'static>> {
    vec![
        Line::from(format!(
            "Максимальный гриф секретности: {}",
            dev.max_secclass.as_deref().unwrap_or("-")
        ))
        .style(Color::Red)
        .bold(),
    ]
}
