use crossterm::event::{self, Event};
use ratatui::{Terminal, prelude::Backend};

use crate::{
    handlers::{app::AppHandler, input::InputHandler},
    models::state::State,
    ui,
};

pub struct App {
    state: State,
    app_handler: AppHandler,
}

impl App {
    pub fn new() -> color_eyre::Result<Self> {
        let app_handler = AppHandler::new()?;
        let mut state = State::new()?;
        state.input = "start
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
end"
        .into();

        // app_handler.load(&mut state);

        Ok(Self { state, app_handler })
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> color_eyre::Result<()> {
        loop {
            terminal.draw(|frame| {
                ui::draw(frame, &self.state);
            })?;

            if let Event::Key(key) = event::read()? {
                if key.is_press() {
                    InputHandler::handle_key_event(&mut self.state, key, &self.app_handler)?;
                }
            }

            if self.state.should_quit {
                break;
            }
        }

        Ok(())
    }
}
