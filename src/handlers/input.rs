use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    handlers::app::AppHandler,
    models::state::{AppState, InputState, State},
};

pub struct InputHandler;

impl InputHandler {
    pub fn handle_key_event(
        state: &mut State,
        key: KeyEvent,
        app_handler: &AppHandler,
    ) -> color_eyre::Result<()> {
        Self::handle_default_inputs(key, state);
        Ok(())
    }

    fn handle_default_inputs(key: KeyEvent, state: &mut State) {
        if !key.is_press() {
            return;
        }

        match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Char('q')) => state.should_quit = true,
            (KeyModifiers::NONE, KeyCode::Tab) => {
                state.app_state = match state.app_state {
                    AppState::LeftSelected(_) => AppState::RightSelected,
                    AppState::RightSelected => AppState::LeftSelected(InputState::Normal),
                }
            }
            _ => {}
        };
    }
}
