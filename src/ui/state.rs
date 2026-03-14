use bevy::prelude::*;
use super::dialog_stack::DialogStack;

/// This file contains the state of the UI, such as whether the pause menu is open,
/// whether the inventory is open, and the stack of dialogs that are currently open.

/// The `UiInitState` is responsible to block further initialization until the ui 
/// basics are initialized
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UiInitState {
    #[default]
    Loading,
    Ready,
}

/// The `UiState` resource is used to track the current state of the UI,
/// including the input mode and the dialog stack.
#[derive(Resource, Default)]
pub struct UiState {
    pub input_mode: InputMode,
    pub dialog_stack: DialogStack,
}

#[derive(Clone, PartialEq, Eq)]
pub enum InputMode {
    Gameplay,
    Menu,
    Dialog,
}

impl Default for InputMode {
    fn default() -> Self {
        InputMode::Gameplay
    }
}


/// attach to player with>
/// command.spawn(( Player, PlayerControl { enabled: true} ))
#[derive(Component)]
pub struct PlayerControl {
    pub enabled: bool,
}


pub fn input_routing_system(ui_state: Res<UiState>, mut gameplay_query: Query<&mut PlayerControl>) {
    match ui_state.input_mode {
        InputMode::Gameplay => {
            for mut control in &mut gameplay_query {
                control.enabled = true;
            }
        }
        InputMode::Dialog | InputMode::Menu => {
            for mut control in &mut gameplay_query {
                control.enabled = false;
            }
        }
    }
}