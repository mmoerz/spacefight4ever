use bevy::prelude::*;
use std::str::FromStr;

/// part of what a button is, the other part is the button component
#[repr(usize)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
    Disabled,
}

impl ButtonState {
    #[inline]
    pub fn index(self) -> usize {
        self as usize
    }
}

/// actual size of UiWindowState - count of windowstates
pub const WINDOW_STATE_COUNT: usize = 5;

/// part of what a window is
#[repr(usize)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum UiWindowState {
    Normal,
    Minimized,
    Maximized,
    Closed,
    Disabled,
    //Focused, // not a good idea, min/max/normal info is overwritten
}

impl UiWindowState {
    #[inline]
    pub fn index(self) -> usize {
        self as usize
    }
}

/// This is an enum with bitwise | or able values so that we can define
/// a list of buttons in a variable.
#[repr(usize)]
#[derive(Component, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum UiButtonType {
    None = 0,
    Menu = 1,
    Minimize = 2,
    Maximize = 4,
    Close = 8,
    Ok = 1 << 7,
    Cancel = 1 << 8,
    Yes = 1 << 5,
    No = 1 << 6,
    Abort = 1 << 9,
    Retry = 1 << 10,
    Ignore = 1 << 11,
    Custom = 1 << 12,
}

/// enable loading from string for the UiButtonType
impl FromStr for UiButtonType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "close" => Ok(UiButtonType::Close),
            "minimize" => Ok(UiButtonType::Minimize),
            "maximize" => Ok(UiButtonType::Maximize),
            "menu" => Ok(UiButtonType::Menu),
            _ => Err(()),
        }
    }
}

/// create an index from the type
impl UiButtonType {
    pub fn index(self) -> usize {
        (self as usize).trailing_zeros() as usize
    }
}

#[derive(Component, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum UiWindowType {
    Simple,
    Standard,
}

/// enable loading from string for the UiWindowType
impl FromStr for UiWindowType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "standard" => Ok(UiWindowType::Standard),
            _ => Err(()),
        }
    }
}
