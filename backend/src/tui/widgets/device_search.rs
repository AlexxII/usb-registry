use crossterm::event::{Event, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
// use ratatui::style::palette::tailwind::SLATE;
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, ToSpan};
use ratatui::widgets::{Block, Paragraph};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

pub struct DeviceSearch {
    pub input: Input,
    pub input_mode: InputMode,
}

#[derive(Debug, Default, PartialEq)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

impl DeviceSearch {
    // const TEXT_COLOR: Color = SLATE.c400;

    pub fn new() -> Self {
        Self {
            input: Input::new("".to_string()),
            input_mode: InputMode::Normal,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let [header_area, input_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(3)]).areas(area);

        self.render_help_message(frame, header_area);
        self.render_input(frame, input_area);
    }

    pub fn handle_event(&mut self, event: &Event) -> Option<String> {
        let Event::Key(key) = event else {
            return None;
        };
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('e') => {
                    self.start_editing();
                    None
                }
                _ => None,
            },
            InputMode::Editing => match key.code {
                KeyCode::Esc => {
                    self.stop_editing();
                    Some(self.input.value().to_string())
                }
                KeyCode::Enter => {
                    self.stop_editing();
                    Some(self.input.value().to_string())
                }
                _ => {
                    self.input.handle_event(event);
                    Some(self.input.value().to_string())
                }
            },
        }
    }

    fn start_editing(&mut self) {
        self.input_mode = InputMode::Editing
    }

    fn stop_editing(&mut self) {
        self.input_mode = InputMode::Normal
    }

    fn render_help_message(&self, frame: &mut Frame, area: Rect) {
        let help_message = Line::from_iter(match self.input_mode {
            InputMode::Normal => [
                "Жми ".to_span(),
                "e".bold(),
                " чтобы начать ввод.".to_span(),
            ],
            InputMode::Editing => [
                "Жми ".to_span(),
                "Esc или Enter".bold(),
                " чтобы закончить.".to_span(),
            ],
        });
        frame.render_widget(help_message, area);
    }

    fn render_input(&self, frame: &mut Frame, area: Rect) {
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let style = match self.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Color::Yellow.into(),
        };
        let input = Paragraph::new(self.input.value())
            .style(style)
            .scroll((0, scroll as u16))
            .block(Block::bordered().title("Поиск"));
        frame.render_widget(input, area);

        if self.input_mode == InputMode::Editing {
            let x = self.input.visual_cursor().max(scroll) - scroll + 1;
            frame.set_cursor_position((area.x + x as u16, area.y + 1))
        }
    }
}
