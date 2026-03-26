use bevy::prelude::*;

use super::structs::{UiElementSize, UiWindowStatus, ResizeSide};

/// window marker
#[derive(Default, Component)]
pub struct UiWindow;

/// title bar marker
#[derive(Default, Component)]
pub struct UiWindowTitleBar;

/// main content marker
#[derive(Default, Component)]
pub struct UiWindowMain;

/// menu button marker
#[derive(Default, Component)]
pub struct UiWindowMenuButton;

/// minimize button marker
#[derive(Default, Component)]
pub struct UiWindowMinimizeButton;

/// maximize button marker
#[derive(Default, Component)]
pub struct UiWindowMaximizeButton;

/// close button marker
#[derive(Default, Component)]
pub struct UiWindowCloseButton;

/// status bar marker
#[derive(Default, Component)]
pub struct UiWindowStatusBar;

#[derive(Component)]
pub struct UiAtlasButtonIndex(pub usize);

/// stores data about the window size, position, etc.
#[derive(Default, Component, Clone, Copy)]
pub struct UiWindowState {
    pub ui_size: UiElementSize,
    pub status: UiWindowStatus,
    pub focused: bool,
    pub normal_size: UiRect,
}

/// easier access to focus
impl UiWindowState {
    pub fn set_focus(&mut self, focused: bool) {
        self.focused = focused;
    }
}

/// what side of the window gets resized
#[derive(Component)]
pub struct UiWindowResizeHandle {
    pub side: ResizeSide,
}
