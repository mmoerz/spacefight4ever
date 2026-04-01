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
pub enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Closed,
    Disabled,
    Focused,
}

impl WindowState {
    #[inline]
    pub fn index(self) -> usize {
        self as usize
    }
}

#[repr(usize)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum TitlebarButtons {
    Menu = 1,
    Minimize = 2,
    Maximize = 4,
    Close = 8
}

