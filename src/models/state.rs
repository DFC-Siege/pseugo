use serde::{Deserialize, Serialize};

use crate::models::nodes::node::Node;

#[derive(Serialize, Deserialize, Debug)]
pub enum InputState {
    Normal,
    Input,
    Visual,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScrollState;

#[derive(Serialize, Deserialize, Debug)]
pub enum AppState {
    LeftSelected(InputState),
    RightSelected,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub input: String,
    pub should_quit: bool,
    pub app_state: AppState,
}

impl State {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {
            input: String::new(),
            should_quit: false,
            app_state: AppState::LeftSelected(InputState::Normal),
        })
    }

    pub fn get_output(&self) -> color_eyre::Result<Node> {
        Node::new(&self.input)
    }
}
