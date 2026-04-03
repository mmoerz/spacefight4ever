use bevy::prelude::*;
use super::theme::UiTheme;

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