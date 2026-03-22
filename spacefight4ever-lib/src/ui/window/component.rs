use bevy::prelude::*;

use super::structs::UiElementSize;

#[derive(Default, Component)]
pub struct UiWindow;

#[derive(Default, Component)]
pub struct UiWindowTitleBar;

#[derive(Default, Component)]
pub struct UiWindowResizeHandle;

#[derive(Default, Component)]
pub struct UiWindowMinimizeButton;

#[derive(Default, Component)]
pub struct UiWindowMaximizeButton;

// state components
//
//

#[derive(Default, Component)]
pub struct UiWindowState {
    pub size: UiElementSize,
    pub minimized: bool,
    pub maximized: bool,
    pub focused: bool,
}

#[derive(Default, Component)]
pub struct UiWindowDrag {
    pub offset: Vec2,
}

#[derive(Default, Component)]
pub struct UiWindowResize {
    pub start_size: Vec2,
    pub start_pos: Vec2,
}