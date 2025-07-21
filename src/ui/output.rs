use edtui::EditorState;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::models::state::{AppState, State};

pub fn render(frame: &mut Frame, rect: &Rect, state: &State, editor_state: &EditorState) {
    let color = match state.app_state {
        AppState::RightSelected => Color::Yellow,
        _ => Color::White,
    };
    let block = Block::new()
        .style(Style::default().fg(color))
        .borders(Borders::all())
        .title_top("preview");
    let text: String = editor_state.lines.clone().into();
    let text: String = match State::get_output(text.as_str()) {
        Ok(node) => node.to_string(),
        Err(e) => e.to_string(),
    };
    let styled_text: Text = text
        .lines()
        .map(|line| Line::from(line).blue())
        .collect::<Vec<Line>>()
        .into();
    Paragraph::new(styled_text)
        .block(block)
        .render(*rect, frame.buffer_mut());
}
