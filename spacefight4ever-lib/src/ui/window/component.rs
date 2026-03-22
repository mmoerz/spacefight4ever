use bevy::prelude::*;

use super::structs::UiElementSize;

#[derive(Default, Component)]
pub struct UiWindow;

#[derive(Default, Component)]
pub struct UiWindowTitleBar;

#[derive(Default, Component)]
pub struct UiWindowMain;

#[derive(Default, Component)]
pub struct UiWindowResizeHandle;

#[derive(Default, Component)]
pub struct UiWindowMenuButton;

#[derive(Default, Component)]
pub struct UiWindowMinimizeButton;

#[derive(Default, Component)]
pub struct UiWindowMaximizeButton;

#[derive(Default, Component)]
pub struct UiWindowCloseButton;

// state components
//
//

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum UiWindowStatus {
    #[default]
    Normal,
    Minimized,
    Maximized,
}

#[derive(Default, Component, Clone, Copy)]
pub struct UiWindowState {
    pub ui_size: UiElementSize,
    pub status: UiWindowStatus,
    pub focused: bool,
    pub normal_size: UiRect,
}

impl UiWindowState {
    pub fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }
}

#[derive(Default, Component, PartialEq, Eq)]
pub enum UiImageButtonState {
    #[default]
    Normal,
    Hover,
    Disabled,
}

impl UiImageButtonState {
    pub fn index(self) -> usize {
        match self {
            UiImageButtonState::Normal => 0,
            UiImageButtonState::Hover => 1,
            UiImageButtonState::Disabled => 2,
        }
    }
}


#[derive(Default, Component)]
pub struct UiWindowResize {
    pub start_size: Vec2,
    pub start_pos: Vec2,
}