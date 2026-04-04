use std::str::FromStr;
use bevy::prelude::*;

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

/// we need bitwise | or able values so that we can define
/// a list of buttons in a variable
#[repr(usize)]
#[derive(Component, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum UiButtonType {
    Menu = 1,
    Minimize = 2,
    Maximize = 4,
    Close = 8
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
    Standard
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

// state components for last used window size-index
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