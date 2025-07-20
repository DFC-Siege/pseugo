use crate::models::nodes::node::Node;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::Rect,
    style::Stylize,
    text::{Line, Text},
    widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

#[derive(Debug, Default)]
pub struct App {
    running: bool,
    scroll: u16,
    content_height: u16,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: false,
            scroll: 0,
            content_height: 0,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    fn render(&mut self, frame: &mut Frame) {
        let title = Line::from("Ratatui Simple Template")
            .bold()
            .blue()
            .centered();

        let pseudocode = "
        start
        if x > 5
            -- test --
            assign y = x + 10
            while counter < 100
                assign counter = counter + 1
                if counter == 50
                    return counter
                elseif counter > 75
                    assign x = x - 1
                else
                    assign y = y * 2
                end
            end
            return y
        else
            assign result = function(x,y,z)
            return !result
        end
        ";

        let text = match Node::new(pseudocode) {
            Ok(_start) => format!("{_start}"),
            Err(e) => {
                eprintln!("Parse error: {e}");
                "Parse failed - see console".to_string()
            }
        };
        let styled_text: Text = text
            .lines()
            .map(|line| Line::from(line).blue())
            .collect::<Vec<Line>>()
            .into();

        let line_count = styled_text.lines.len() as u16;
        self.content_height = line_count;

        let main_area = frame.area();
        let content_area = Rect {
            width: main_area.width.saturating_sub(1),
            ..main_area
        };
        let scrollbar_area = Rect {
            x: main_area.x + main_area.width.saturating_sub(1),
            y: main_area.y,
            width: 1,
            height: main_area.height,
        };

        let paragraph = Paragraph::new(styled_text)
            .block(Block::bordered().title(title))
            .scroll((self.scroll, 0));

        frame.render_widget(paragraph, content_area);

        let visible_lines = content_area.height.saturating_sub(2);
        if line_count > visible_lines {
            let mut scrollbar_state = ScrollbarState::default()
                .content_length(line_count as usize)
                .viewport_content_length(visible_lines as usize)
                .position(self.scroll as usize);

            let scrollbar = Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓"));

            frame.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
        }
    }

    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),

            // Scrolling controls
            (_, KeyCode::Up | KeyCode::Char('k')) => self.scroll_up(),
            (_, KeyCode::Down | KeyCode::Char('j')) => self.scroll_down(),
            (_, KeyCode::PageUp) => self.page_up(),
            (_, KeyCode::PageDown) => self.page_down(),
            (_, KeyCode::Home) => self.scroll_to_top(),
            (_, KeyCode::End) => self.scroll_to_bottom(),

            _ => {}
        }
    }

    fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    fn scroll_down(&mut self) {
        let max_scroll = self.get_max_scroll();
        if self.scroll < max_scroll {
            self.scroll += 1;
        }
    }

    fn page_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(10);
    }

    fn page_down(&mut self) {
        let max_scroll = self.get_max_scroll();
        self.scroll = (self.scroll + 10).min(max_scroll);
    }

    fn scroll_to_top(&mut self) {
        self.scroll = 0;
    }

    fn scroll_to_bottom(&mut self) {
        self.scroll = self.get_max_scroll();
    }

    fn get_max_scroll(&self) -> u16 {
        // Approximate visible lines (this could be more accurate with terminal size)
        let visible_lines = 20; // You might want to calculate this based on terminal size
        self.content_height.saturating_sub(visible_lines)
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
