use edtui::EditorState;
use serde::{Deserialize, Serialize};

use crate::models::nodes::node::Node;

#[derive(Serialize, Deserialize, Debug)]
pub struct ScrollState;

#[derive(Serialize, Deserialize, Debug)]
pub enum AppState {
    LeftSelected,
    RightSelected,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub should_quit: bool,
    pub app_state: AppState,
    #[serde(skip)]
    pub editor_state: EditorState,
}

impl State {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            should_quit: false,
            app_state: AppState::LeftSelected,
            editor_state: EditorState::default(),
        })
    }

    pub fn get_output(text: &str) -> color_eyre::Result<Node> {
        Node::new(text)
    }
}
