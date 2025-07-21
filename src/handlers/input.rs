use edtui::{EditorEventHandler, EditorMode, EditorState};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{
    handlers::app::AppHandler,
    models::state::{AppState, State},
};

pub struct InputHandler;

impl InputHandler {
    pub fn handle_key_event(
        state: &mut State,
        editor_state: &mut EditorState,
        key: KeyEvent,
        app_handler: &AppHandler,
        event_handler: &mut EditorEventHandler,
    ) -> color_eyre::Result<()> {
        match &state.app_state {
            AppState::LeftSelected => {
                if editor_state.mode == EditorMode::Normal && Self::handle_normal_state(key, state)
                {
                    return Ok(());
                }
                event_handler.on_key_event(key, editor_state);
            }
            AppState::RightSelected => Self::handle_default_inputs(key, state),
        };
        Ok(())
    }

    fn handle_default_inputs(key: KeyEvent, state: &mut State) {
        match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Char('q')) => state.should_quit = true,
            (KeyModifiers::NONE, KeyCode::Tab) => {
                state.app_state = match state.app_state {
                    AppState::LeftSelected => AppState::RightSelected,
                    AppState::RightSelected => AppState::LeftSelected,
                }
            }
            _ => {}
        };
    }

    fn handle_normal_state(key: KeyEvent, state: &mut State) -> bool {
        match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Char('q')) => {
                state.should_quit = true;
                true
            }
            (KeyModifiers::NONE, KeyCode::Tab) => {
                state.app_state = match state.app_state {
                    AppState::LeftSelected => AppState::RightSelected,
                    AppState::RightSelected => AppState::LeftSelected,
                };
                true
            }
            _ => false,
        }
    }
}
