use edtui::{EditorEventHandler, EditorState, Lines};
use ratatui::crossterm::event::{self, Event, KeyEventKind};
use ratatui::{Terminal, prelude::Backend};

use crate::{
    handlers::{app::AppHandler, input::InputHandler},
    models::state::State,
    ui,
};

pub struct App {
    state: State,
    app_handler: AppHandler,
    event_handler: EditorEventHandler,
    editor_state: EditorState,
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        let app_handler = AppHandler::new()?;
        let state = State::new()?;
        let event_handler = EditorEventHandler::default();
        let mut editor_state = EditorState::default();
        editor_state.lines = Lines::from(
            "start
         if x > 5
             -- test --
             let y = x + 10
             while counter < 100
        let counter = counter + 1
        if counter == 50
            return counter
        elseif counter > 75
            let x = x - 1
        else
            let y = y * 2
        end
             end
             return y
         else
             let result = test(x, y, z)
             return !result
         end",
        );

        // app_handler.load(&mut state);

        Ok(Self {
            state,
            app_handler,
            event_handler,
            editor_state,
        })
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> color_eyre::Result<()> {
        loop {
            terminal.draw(|frame| {
                ui::draw(frame, &self.state, &mut self.editor_state);
            })?;

            let event = event::read()?;

            if let Event::Key(key) = event
            {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                InputHandler::handle_key_event(
                    &mut self.state,
                    &mut self.editor_state,
                    key,
                    &self.app_handler,
                    &mut self.event_handler,
                )?;
            };

            if self.state.should_quit {
                return Ok(());
            }
        }
    }
}
