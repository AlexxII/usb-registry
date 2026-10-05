use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Widget};

use crate::models::device::MappedDevice;

pub struct DeviceInfo;

impl DeviceInfo {
    pub fn render(
        device: Option<&MappedDevice>,
        area: Rect,
        buf: &mut Buffer,
        ex: bool,
        query: Option<&str>,
    ) {
        let block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(1, 0, 1, 0))
            .title(" Детальная информация ");

        let Some(dev) = device else {
            let placeholder = Paragraph::new("Выберите устройство для просмотра деталей...")
                .alignment(ratatui::layout::Alignment::Center).block(block);
            placeholder.render(area, buf);
            return;
        };

        // Формируем текст с детальной информацией
        let text = device_info(&dev, ex, query);


        let paragraph = Paragraph::new(text).block(block);
        paragraph.render(area, buf);
    }
}

fn device_info(dev: &MappedDevice, ex: bool, query: Option<&str>) -> Vec<Line<'static>> {
    let register_info = register_info(dev, query);
    let special_dev = is_special(dev, ex);
    let internet = is_internet(dev, ex, query);
    let max_secclas = show_maxsecclass(dev);

    let mut lines = vec![
        highlighted_field(
            "Производитель: ",
            dev.manufacturer.as_deref().unwrap_or("Неизвестно"),
            query,
        ),
        highlighted_field(
            "Серийный номер: ",
            dev.serial.as_deref().unwrap_or("-"),
            query,
        ),
        highlighted_field(
            "Файловая система: ",
            dev.filesystem.as_deref().unwrap_or("-"),
            query,
        ),
        highlighted_field("Объем: ", dev.capacity.as_deref().unwrap_or("-"), query),
    ];

    if dev.registered & dev.secret {
        lines.extend(register_info);
        lines.extend(special_dev);
        lines.extend(max_secclas);

        if dev.destroyed {
            lines.push(Line::from("Носитель УНИЧТОЖЕН").style(Color::Red));
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

fn register_info(dev: &MappedDevice, query: Option<&str>) -> Vec<Line<'static>> {
    if dev.registered {
        return vec![
            Line::from("ЗАРЕГИСТРИРОВАН").style(Color::Green).bold(),
            highlighted_field(
                "Рег.№: ",
                dev.register_number.as_deref().unwrap_or("-"),
                query,
            ),
            highlighted_field("Владелец: ", dev.owner.as_deref().unwrap_or("-"), query),
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

fn is_special(dev: &MappedDevice, ex: bool) -> Vec<Line<'static>> {
    let mut lines = vec![];
    if dev.special {
        lines.push(
            Line::from("СПЕЦИАЛЬНОЕ ДЕЛОПРОИЗВОДСТВО")
                .style(Color::Red)
                .bold(),
        );
        if ex {
            lines.push(
                Line::from(
                    "если ваш диск не зарегистрирован на участке СПЕЦИАЛЬНОГО делопроизводства - Вы попали!",
                )
            );
        }
    }
    lines
}

fn is_internet(dev: &MappedDevice, ex: bool, query: Option<&str>) -> Vec<Line<'static>> {
    let mut lines = vec![];
    if !dev.secret {
        lines.push(Line::from("ЗАРЕГИСТРИРОВАН").style(Color::Green).bold());
        lines.push(highlighted_field(
            "Рег.№: ",
            dev.register_number.as_deref().unwrap_or("-"),
            query,
        ));
        lines.push(highlighted_field(
            "Владелец: ",
            dev.owner.as_deref().unwrap_or("-"),
            query,
        ));
        lines.push(Line::from("ДЛЯ АП ИНТЕРНЕТ").style(Color::Red).bold());
        if ex {
            lines.push(Line::from(
                "Если вы сидите за ОВТ и видите это сообщение - Вы попали!",
            ));
        }
    }
    lines
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

fn highlight_matches(text: &str, query: &str) -> Vec<Span<'static>> {
    let query = query.trim();

    if query.chars().count() < 2 {
        return vec![Span::raw(text.to_owned())];
    }

    let query_lower = query.to_lowercase();

    let mut result = Vec::new();
    let mut current = String::new();

    for ch in text.chars() {
        current.push(ch);

        let current_lower = current.to_lowercase();

        if current_lower.ends_with(&query_lower) {
            let match_len = query.chars().count();

            let prefix_len = current.chars().count() - match_len;

            let prefix: String = current.chars().take(prefix_len).collect();

            let matched: String = current.chars().skip(prefix_len).collect();

            if !prefix.is_empty() {
                result.push(Span::raw(prefix));
            }

            result.push(Span::styled(
                matched,
                Style::default().fg(Color::Yellow).bold(),
            ));

            current.clear();
        }
    }

    if !current.is_empty() {
        result.push(Span::raw(current));
    }

    result
}

fn highlighted_field(label: &'static str, value: &str, query: Option<&str>) -> Line<'static> {
    let mut spans = vec![Span::raw(label)];

    match query {
        Some(query) if query.len() >= 2 => {
            spans.extend(highlight_matches(value, query));
        }
        _ => {
            spans.push(Span::raw(value.to_owned()));
        }
    }

    Line::from(spans)
}
