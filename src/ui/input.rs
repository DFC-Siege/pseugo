use color_eyre::owo_colors::{FgColorDisplay, OwoColorize};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::models::state::{AppState, State};

pub fn render(frame: &mut Frame, rect: &Rect, state: &State) {
    let color = match state.app_state {
        AppState::LeftSelected(_) => Color::Yellow,
        _ => Color::White,
    };
    let block = Block::new()
        .style(Style::default().fg(color))
        .borders(Borders::all())
        .title_top("edit");
    let text = state.input.to_string();
    let styled_text: Text = text
        .lines()
        .map(|line| Line::from(line).white())
        .collect::<Vec<Line>>()
        .into();
    Paragraph::new(styled_text)
        .block(block)
        .render(*rect, frame.buffer_mut());
}
