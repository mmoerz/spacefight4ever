use bevy::prelude::*;

#[derive(Message)]
pub enum DialogRequest {
    ConfirmExit,
    Message(String),
}

#[derive(Message)]
pub enum DialogResult {
    ConfirmExit(bool),
}