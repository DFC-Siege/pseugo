use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::{
    models::state::State,
    ui::{input, output},
};

pub fn draw_main(frame: &mut Frame, rect: Rect, state: &State) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rect);

    input::render(frame, &layout[0], state);
    output::render(frame, &layout[1], state);
}
