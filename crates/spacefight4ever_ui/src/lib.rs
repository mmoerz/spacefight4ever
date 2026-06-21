pub mod bundle;
pub mod consts;
pub mod structs;

pub mod ui {
    pub mod assets {
        pub mod asseterror;
        pub mod assets;
        pub mod atlasbuttonskin;
        pub mod disktheme;
        pub mod theme;
        pub mod titlebarskin;
        pub mod windowsskin;
    }
    pub mod atlasbutton;
    pub mod button;
    pub mod dialog;
    pub mod progressbar;
    pub mod progressbar_commands;
    pub mod progressbar_material;
    pub mod titlebar;
    pub mod window;
}

pub mod prelude {
    pub use crate::bundle::*;
    pub use crate::consts::*;
    pub use crate::structs::*;
    pub use crate::ui::assets::assets::setup_ui_theme;
    pub use crate::ui::assets::theme::UiTheme;
    pub use crate::ui::dialog::{
        UiDialog, UiDialogBuilder, UiDialogClosed, UiDialogEvent, UiDialogOpened,
        spawn_confirm_dialog, spawn_message_dialog, spawn_ok_cancel_dialog,
    };
    pub use crate::ui::window::{
        UiAtlasWindow, UiWindowCurrentState, UiWindowFocused, UiWindowZCounter, spawn_ui_window,
        spawn_ui_window_with_z_index,
    };
}

pub use bundle::*;

// for asset plugin
// use ui::assets::{
//     //atlasbuttonskin::{ButtonSkinLoader, ButtonSkin},
//     //windowsskin::{WindowSkinLoader, WindowSkin},
//     //theme::UiTheme,
//     //disktheme::UiThemeLoader
// };

pub mod plugins {
    pub use crate::ui::{
        assets::assets::UiAssetsPlugin, atlasbutton::UiAtlasButtonPlugin, dialog::UiDialogPlugin,
        progressbar_material::UiProgressBarPlugin, titlebar::UiTitleBarPlugin,
        window::UiAtlasWindowPlugin,
    };
}
