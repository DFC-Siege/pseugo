use edtui::EditorState;
use ratatui::Frame;

use crate::models::state::State;

mod app;
mod input;
mod output;

pub fn draw(frame: &mut Frame, state: &State, editor_state: &mut EditorState) {
    app::draw_main(frame, frame.area(), state, editor_state);
}
