use bevy::prelude::*;

pub mod consts;
pub mod structs;
pub mod bundle;

pub mod ui {
    pub mod assets {
        pub mod asseterror;
        pub mod atlasbuttonskin;
        pub mod titlebarskin;
        pub mod windowsskin;
        pub mod disktheme;
        pub mod theme;
        pub mod assets;
    }
    pub mod button;
    pub mod atlasbutton;
    pub mod titlebar;
    pub mod window;
    pub mod progressbar;
}

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::structs::*;
    pub use crate::bundle::*;
    pub use crate::ui::assets::theme::UiTheme;
    pub use crate::ui::assets::assets::setup_ui_theme;
    pub use crate::ui::window::{UiAtlasWindow, UiWindowFocused, UiWindowCurrentState, UiWindowZCounter, spawn_ui_window, spawn_ui_window_with_z_index};
}

pub use bundle::*;

// for asset plugin
use ui::assets::{
    atlasbuttonskin::{ButtonSkinLoader, ButtonSkin},
    windowsskin::{WindowSkinLoader, WindowSkin},
    theme::UiTheme,
    disktheme::UiThemeLoader
};

pub mod plugins {
    pub use crate::ui::{
        assets::assets::UiAssetsPlugin,
        atlasbutton::UiAtlasButtonPlugin,
        titlebar::UiTitleBarPlugin,
        window::UiAtlasWindowPlugin,
        progressbar::UiProgressBarPlugin,
    };
}