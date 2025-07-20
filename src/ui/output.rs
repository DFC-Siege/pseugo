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
        AppState::RightSelected => Color::Yellow,
        _ => Color::White,
    };
    let block = Block::new()
        .style(Style::default().fg(color))
        .borders(Borders::all())
        .title_top("preview");
    let text = match state.get_output() {
        Ok(start) => format!("{start}"),
        Err(_) => "".to_string(),
    };
    let styled_text: Text = text
        .lines()
        .map(|line| Line::from(line).blue())
        .collect::<Vec<Line>>()
        .into();
    Paragraph::new(styled_text)
        .block(block)
        .render(*rect, frame.buffer_mut());
    // let output_block = Block::default()
    //     .title("Output")
    //     .borders(Borders::ALL)
    //     .border_style(if !input_mode {
    //         Style::default().fg(Color::Yellow)
    //     } else {
    //         Style::default().fg(Color::Gray)
    //     });
    //
    // let content_area = Rect {
    //     x: area.x + 1,
    //     y: area.y + 1,
    //     width: area.width.saturating_sub(3), // Leave space for scrollbar
    //     height: area.height.saturating_sub(2),
    // };
    //
    // let scrollbar_area = Rect {
    //     x: area.x + area.width.saturating_sub(2),
    //     y: area.y + 1,
    //     width: 1,
    //     height: area.height.saturating_sub(2),
    // };
    //
    // let code_text = if input.trim().is_empty() {
    //     "
    //     start
    //     if x > 5
    //         -- test --
    //         assign y = x + 10
    //         while counter < 100
    //             assign counter = counter + 1
    //             if counter == 50
    //                 return counter
    //             elseif counter > 75
    //                 assign x = x - 1
    //             else
    //                 assign y = y * 2
    //             end
    //         end
    //         return y
    //     else
    //         assign result = function(x,y,z)
    //         return !result
    //     end
    //     "
    // } else {
    //     &input
    // };
    //
    // let text = match Node::new(code_text) {
    //     Ok(start) => format!("{start}"),
    //     Err(_) => "".to_string(),
    // };
    //
    // let styled_text: Text = text
    //     .lines()
    //     .map(|line| Line::from(line).blue())
    //     .collect::<Vec<Line>>()
    //     .into();
    //
    // let line_count = styled_text.lines.len() as u16;
    // content_height = line_count;
    //
    // let paragraph = Paragraph::new(styled_text)
    //     .block(output_block)
    //     .scroll((scroll, 0));
    //
    // frame.render_widget(paragraph, area);
    //
    // let visible_lines = content_area.height;
    // if line_count > visible_lines {
    //     let mut scrollbar_state = ScrollbarState::default()
    //         .content_length(line_count as usize)
    //         .viewport_content_length(visible_lines as usize)
    //         .position(scroll as usize);
    //
    //     let scrollbar = Scrollbar::default()
    //         .orientation(ScrollbarOrientation::VerticalRight)
    //         .begin_symbol(Some("↑"))
    //         .end_symbol(Some("↓"));
    //
    //     frame.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    // }
}
