use ratatui::prelude::*;
use std::collections::HashMap;

// Высота нашего шрифта в терминальных строках
const FONT_HEIGHT: usize = 6;
// Ширина пробела между буквами
const LETTER_SPACING: &str = " ";

/// Возвращает карту готовых глифов.
/// Использование псевдографики '█' делает буквы монолитными и яркими.
fn get_cyrillic_font() -> HashMap<char, Vec<&'static str>> {
    let mut font = HashMap::new();

    font.insert(
        'А',
        vec![
            "  ██████  ",
            " ██    ██ ",
            " ████████ ",
            " ██    ██ ",
            " ██    ██ ",
            "          "
        ],
    );
    font.insert(
        'Б',
        vec![
            " ███████ ",
            " ██      ",
            " ███████ ",
            " ██   ██ ",
            " ███████ ",
            "         "
        ],
    );
    font.insert(
        'В',
        vec![
            " ██████  ",
            " ██   ██ ",
            " ██████  ",
            " ██   ██ ",
            " ██████  ",
            "         "
        ],
    );
    font.insert(
        'Г',
        vec![
            " ████████ ",
            " ██       ",
            " ██       ",
            " ██       ",
            " ██       ",
            "          "
        ],
    );
    font.insert(
        'Д',
        vec![
            "   ████   ",
            "  ██  ██  ",
            " ██    ██ ",
            " ████████ ",
            " ██    ██ ", 
            "          "
        ],
    );
    font.insert(
        'Е',
        vec![
            " ████████ ",
            " ██       ",
            " ██████   ",
            " ██       ",
            " ████████ ",
            "          "
        ],
    );
    font.insert(
        'Ж',
        vec![
            " ██   ██   ██ ",
            "  ██  ██  ██  ",
            "   ████████   ",
            "  ██  ██  ██  ",
            " ██   ██   ██ ",
            "              "
        ],
    );
    font.insert(
        'З',
        vec![
            "  ██████  ",
            " ██    ██ ",
            "     ████ ",
            " ██    ██ ",
            "  ██████  ",
            "          "
        ],
    );
    font.insert(
        'И',
        vec![
            " ██    ██ ",
            " ██  ████ ",
            " ██ ██ ██ ",
            " ████  ██ ",
            " ██    ██ ",
            "          "
        ],
    );
    font.insert(
        'К',
        vec![
            " ██   ██ ",
            " ██  ██  ",
            " █████   ",
            " ██  ██  ",
            " ██   ██ ",
            "         "
        ],
    );
    font.insert(
        'Л',
        vec![
            "   ████   ",
            "  ██  ██  ",
            " ██    ██ ",
            " ██    ██ ",
            " ██    ██ ",
            "          "
        ],
    );
    font.insert(
        'М',
        vec![
            " ██      ██ ",
            " ████  ████ ",
            " ██ ████ ██ ",
            " ██  ██  ██ ",
            " ██      ██ ",
            "            "
        ],
    );
    font.insert(
        'Н',
        vec![
            " ██    ██ ",
            " ██    ██ ",
            " ████████ ",
            " ██    ██ ",
            " ██    ██ ",
            "          "
        ],
    );
    font.insert(
        'О',
        vec![
            "  ██████  ",
            " ██    ██ ",
            " ██    ██ ",
            " ██    ██ ",
            "  ██████  ",
            "          "
        ],
    );
    font.insert(
        'П',
        vec![
            " ████████ ",
            " ██    ██ ",
            " ██    ██ ",
            " ██    ██ ",
            " ██    ██ ",
            "          "
        ],
    );
    font.insert(
        'Р',
        vec![
            " ███████ ",
            " ██   ██ ",
            " ███████ ",
            " ██      ",
            " ██      ",
            "         "
        ],
    );
    font.insert(
        'С',
        vec![
            "  ██████ ",
            " ██      ",
            " ██      ",
            " ██      ",
            "  ██████ ",
            "         "
        ],
    );
    font.insert(
        'Т',
        vec![
            " ████████ ",
            "    ██    ",
            "    ██    ",
            "    ██    ",
            "    ██    ",
            "          "
        ],
    );
    font.insert(
        'У',
        vec![
            " ██    ██ ",
            " ██    ██ ",
            "  ██████  ",
            "      ██  ",
            "  █████   ",
            "          "
        ],
    );
    font.insert(
        'Ф',
        vec![
            "   ████   ",
            " ██ ██ ██ ",
            " ██ ██ ██ ",
            "   ████   ",
            "    ██    ",
            "          "
        ],
    );
    font.insert(
        'Х',
        vec![
            " ██    ██ ",
            "  ██  ██  ",
            "   ████   ",
            "  ██  ██  ",
            " ██    ██ ",
            "          "
        ],
    );
    font.insert(
        'Ц',
        vec![
            " ██    ██  ",
            " ██    ██  ",
            " ██    ██  ",
            " ██    ██  ",
            " ████████  ",
            "        ██ ",
        ],
    );
    font.insert(
        'Ч',
        vec![
            " ██    ██ ",
            " ██    ██ ",
            " ████████ ",
            "       ██ ",
            "       ██ ",
            "          "
        ],
    );
    font.insert(
        'Ш',
        vec![
            " ██  ██  ██ ",
            " ██  ██  ██ ",
            " ██  ██  ██ ",
            " ██  ██  ██ ",
            " ██████████ ",
            "            "
        ],
    );
    font.insert(
        'Щ',
        vec![
            " ██  ██  ██  ",
            " ██  ██  ██  ",
            " ██  ██  ██  ",
            " ██  ██  ██  ",
            " ██████████  ",
            "          ██ ",
        ],
    );
    font.insert(
        'Ъ',
        vec![
            " ████      ",
            "   ██      ",
            "   ██████  ",
            "   ██   ██ ",
            "   ██████  ",
            "           "
        ],
    );
    font.insert(
        'Ы',
        vec![
            " ██     ██ ",
            " ██     ██ ",
            " ██████ ██ ",
            " ██  ██ ██ ",
            " ██████ ██ ",
            "           "
        ],
    );
    font.insert(
        'Ь',
        vec![
            " ██     ",
            " ██     ",
            " ██████ ",
            " ██  ██ ",
            " ██████ ",
            "        "
        ],
    );
    font.insert(
        'Э',
        vec![
            "  ██████  ",
            "       ██ ",
            "  ███████ ",
            "       ██ ",
            "  ██████  ",
            "          "
        ],
    );
    font.insert(
        'Ю',
        vec![
            " ██    ██████  ",
            " ██   ██    ██ ",
            " ███████    ██ ",
            " ██   ██    ██ ",
            " ██    ██████  ",
            "               "
        ],
    );
    font.insert(
        'Я',
        vec![
            "  ███████ ",
            " ██    ██ ",
            "  ███████ ",
            " ██    ██ ",
            " ██    ██ ",
            "          "
        ],
    );
    // Делаем копии для строчных букв, чтобы регистр не ломал вывод
    let keys: Vec<char> = font.keys().cloned().collect();
    for key in keys {
        if let Some(glyph) = font.get(&key).cloned() {
            font.insert(key.to_lowercase().next().unwrap(), glyph);
        }
    }

    font
}

/// Функция преобразует обычную строку в вектор строк (`Line`) для Ratatui Paragraph
pub fn create_big_text(text: &str, color: Color) -> Vec<Line<'static>> {
    let font = get_cyrillic_font();

    // Создаем заготовку: массив из 5 пустых строк (Lines)
    let mut lines = vec![Line::from(vec![]); FONT_HEIGHT];

    // Стандартная заглушка (пробел), если символа нет в шрифте
    let default_glyph = vec!["      ", "      ", "      ", "      ", "      "];

    for ch in text.chars() {
        let glyph = font.get(&ch).unwrap_or(&default_glyph);

        for i in 0..FONT_HEIGHT {
            // Добавляем саму букву
            lines[i].spans.push(Span::styled(
                glyph[i].to_string(),
                Style::default().fg(color),
            ));
            // Добавляем межбуквенный интервал
            lines[i].spans.push(Span::raw(LETTER_SPACING));
        }
    }

    lines
}
