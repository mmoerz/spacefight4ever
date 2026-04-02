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
        pub mod windowsskin;
        pub mod disktheme;
        pub mod theme;
    }
    pub mod button;
    pub mod atlasbutton;
}

pub mod window {
    pub mod window;
    pub mod window_observers;

    pub mod systems {
        pub mod button;
        pub mod close;
        pub mod minmax;
        pub mod resize;
        pub mod window_atlas_switch;
    }
}

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::structs::*;
    pub use crate::component::*;
    pub use crate::bundle::*;
    pub use crate::resource::*;
    pub use crate::window::window::*;
}

pub use bundle::*;

use crate::resource::*;

use crate::window::window_observers::*;
use crate::window::systems::button::*;
use crate::window::systems::close::*;
use crate::window::systems::minmax::*;
use crate::window::systems::resize::*;

use crate::window::systems::window_atlas_switch::*;

// for asset plugin
use ui::assets::{
    atlasbuttonskin::{ButtonSkinLoader, ButtonSkin},
    windowsskin::{WindowSkinLoader, WindowSkin},
    theme::UiTheme,
    disktheme::UiThemeLoader
};


pub struct UiWindowPlugin;

impl Plugin for UiWindowPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiWindowZCounter>()
            .init_resource::<UiWindowFocused>()
            .init_resource::<UiWindowAtlas>()
            .add_observer(on_window_click_focus)
            .add_observer(on_window_titlebar_drag_start)
            .add_observer(on_window_titlebar_drag)
            .add_observer(on_window_titlebar_drag_end)
            .add_observer(window_resize_system)
            .add_message::<UiWindowsStatusChangeRequest>()
            .add_systems(Startup, setup_window_bundle)
            .add_systems(Update, (
                minimize_windows,
                apply_window_status_change,
                maximize_windows,
                close_windows,
                window_button_interaction_system
            ));
    }
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


/// Plugin for registering all UI assets
pub struct UiAssetsPlugin;

impl Plugin for UiAssetsPlugin {
    fn build(&self, app: &mut App) {
        // Register custom asset types
        app.init_asset::<ButtonSkin>()
           .init_asset_loader::<ButtonSkinLoader>();

        app.init_asset::<WindowSkin>()
           .init_asset_loader::<WindowSkinLoader>();

        app.init_asset::<UiTheme>()
           .init_asset_loader::<UiThemeLoader>();
    }
}


pub fn setup_window_bundle(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let button_atlas_offset: u32 = 5;

    let atlas_layout =
        TextureAtlasLayout::from_grid(UVec2::new(70, 70), 4, 4, Some(UVec2::splat(0)), None);
    let window_atlas_handle = texture_atlases.add(atlas_layout);
    let atlas_layout =
        TextureAtlasLayout::from_grid(UVec2::new(60, 60), button_atlas_offset, 3, Some(UVec2::splat(4)), None);
    let button_atlas_handle = texture_atlases.add(atlas_layout);

    commands.insert_resource(UiWindowAtlas {
        window_layout: window_atlas_handle,
        button_layout: button_atlas_handle,
        button_offset: button_atlas_offset as usize,
    });
}
