use crate::models::state::{AppState, State};
use edtui::{EditorState, EditorTheme, EditorView};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
};

pub fn render(frame: &mut Frame, rect: &Rect, state: &State, editor_state: &mut EditorState) {
    let color = match state.app_state {
        AppState::LeftSelected => Color::Yellow,
        _ => Color::White,
    };
    let block = Block::new()
        .style(Style::default().fg(color))
        .borders(Borders::all())
        .title_top("edit");

    let theme = EditorTheme::default()
        .block(block)
        .base(Style::default().bg(Color::Reset).fg(Color::White));
    EditorView::new(editor_state)
        .theme(theme)
        .wrap(true)
        .render(*rect, frame.buffer_mut());
}
