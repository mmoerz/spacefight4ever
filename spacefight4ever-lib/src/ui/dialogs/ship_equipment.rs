use bevy::prelude::*;
use spacefight4ever_ui::{
    prelude::{UiTheme, UiWindowZCounter, spawn_ui_window_with_z_index},
    ui::{assets::{assets::UiResources, atlasbuttonskin::ButtonSkin, windowsskin::WindowSkin}, button::UiWindowType},
};

pub fn spawn_ship_equipment_dialog(
    commands: &mut Commands,
    parent: Entity,
    asset_server: &Res<AssetServer>,
    z_index: ResMut<UiWindowZCounter>,
    ui_resources: &Res<UiResources>,
    themes: &Assets<UiTheme>,
    button_skins: &Assets<ButtonSkin>, // pass skins here
    window_skins: &Assets<WindowSkin>, // pass skins here
) -> Entity {
    let theme = themes.get(&ui_resources.theme_handle).unwrap();
    let window_id = spawn_ui_window_with_z_index(
        commands, "Ship Equipment".into(), UiWindowType::Standard, theme, button_skins, window_skins, z_index
    );

    // Spawn dialog panel under DialogRoot
    commands
        .entity(parent)
        .add_child(window_id);

    window_id
}