use bevy::prelude::*;

use super::atlasbuttonskin::{ButtonSkin, ButtonSkinLoader};
use super::windowsskin::{WindowSkin, WindowSkinLoader};
use super::theme::UiTheme;
use super::disktheme::UiThemeLoader;

#[derive(Resource, Default, Debug)]
pub struct UiResources {
    pub theme_handle: Handle<UiTheme>,
}

pub fn setup_ui_theme (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let theme_handle: Handle<UiTheme> =
        asset_server.load("themes/standard.uitheme.ron");
    
    // store the handle of the uitheme
    commands.insert_resource(UiResources{
        theme_handle
    });
}

pub struct UiThemePlugin;

impl Plugin for UiThemePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiResources>()
            .add_systems(Startup, setup_ui_theme);
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