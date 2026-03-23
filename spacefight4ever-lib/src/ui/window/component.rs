use bevy::prelude::*;

use super::structs::UiElementSize;

#[derive(Default, Component)]
pub struct UiWindow;

#[derive(Default, Component)]
pub struct UiWindowTitleBar;

#[derive(Default, Component)]
pub struct UiWindowMain;

#[derive(Default, Component)]
pub struct UiWindowMenuButton;

#[derive(Default, Component)]
pub struct UiWindowMinimizeButton;

#[derive(Default, Component)]
pub struct UiWindowMaximizeButton;

#[derive(Default, Component)]
pub struct UiWindowCloseButton;

#[derive(Default, Component)]
pub struct UiWindowStatusBar;


#[derive(Component)]
pub struct UiWindowNinePatch {
    // The sprite handle
    pub texture: Handle<Image>,
    // How much to slice the edges (in pixels)
    pub slice: Vec4, // left, right, top, bottom
}

// state components
//
//
#[derive(Resource, Default, Debug)]
pub struct UiWindowZCounter(i32);

impl UiWindowZCounter {
    pub fn inc(&mut self) -> i32 {
        self.0 += 1;
        self.0
    }
    pub fn get(&self) -> i32 {
        self.0
    }
}

#[derive(Resource, Debug)]
pub struct UiWindowFocused(Entity);

impl Default for UiWindowFocused {
    fn default() -> Self {
        Self(Entity::PLACEHOLDER)
    }
}

impl UiWindowFocused {
    pub fn set(&mut self, entity: Entity) {
        self.0 = entity;
    }
    pub fn get(&self) -> Entity {
        self.0
    }
}

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

#[derive(Component)]
pub struct UiWindowResizeHandle {
    pub side: ResizeSide,
}

#[derive(Clone, Copy)]
pub enum ResizeSide {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
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

