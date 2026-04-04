use bevy::prelude::*;

pub mod consts;
pub mod structs;
pub mod component;
pub mod bundle;
pub mod resource;

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
}

pub mod window {
    pub mod systems {
        pub mod window_atlas_switch;
    }
}

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::structs::*;
    pub use crate::component::*;
    pub use crate::bundle::*;
    pub use crate::resource::*;
    pub use crate::ui::assets::theme::UiTheme;
    pub use crate::ui::assets::assets::setup_ui_theme;
    pub use crate::ui::window::{UiAtlasWindow, UiWindowFocused, UiWindowCurrentState, UiWindowZCounter, ui_window_bundle};
}

pub use bundle::*;

use crate::window::systems::window_atlas_switch::*;

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
    };
}

pub struct UiWindowExtensionPlugin;
impl Plugin for UiWindowExtensionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiWindowAtlasStatus>()
            .add_message::<UiWindowsSwitchAtlasRequest>()
            .add_systems(Update, window_atlas_switch_system);
    }
}
