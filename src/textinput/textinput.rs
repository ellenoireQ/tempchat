use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Default)]
pub struct TextInput {
    pub value: String,
    pub cursor: usize,
    pub focused: bool,
}

impl TextInput {
    pub fn handle_event(&mut self, key: KeyEvent) {
        if !self.focused {
            return;
        }

        match key.code {
            KeyCode::Char(c) => {
                self.value.insert(self.cursor, c);
                self.cursor += 1;
            }
            KeyCode::Backspace => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.value.remove(self.cursor);
                }
            }
            KeyCode::Left => {
                self.cursor = self.cursor.saturating_sub(1);
            }
            KeyCode::Right => {
                self.cursor = (self.cursor + 1).min(self.value.len());
            }
            _ => {}
        }
    }
}

impl Widget for &TextInput {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let style = if self.focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        let display_text = if self.focused {
            let mut text = self.value.clone();
            if self.cursor <= text.len() {
                text.insert(self.cursor, '│');
            }
            text
        } else {
            self.value.clone()
        };

        let paragraph = Paragraph::new(display_text)
            .block(Block::default().borders(Borders::ALL).title("Input"))
            .style(style);

        paragraph.render(area, buf);
    }
}
