use bevy::prelude::*;
use super::theme::UiTheme;

#[derive(Resource, Default, Debug)]
pub struct UiResources {
    pub theme:UiTheme,
}

pub fn setup_ui_theme (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    themes: Res<Assets<UiTheme>>,
) {
    let theme_handle: Handle<UiTheme> =
        asset_server.load("themes/standard.uitheme.ron");
    
    // store the handle of the uitheme
    if let Some(theme) = themes.get(&theme_handle) {
        commands.insert_resource(UiResources{
            theme: theme.clone()
        });
    }
}

pub struct UiThemePlugin;

impl Plugin for UiThemePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiResources>()
            .add_systems(Startup, setup_ui_theme);
    }
}