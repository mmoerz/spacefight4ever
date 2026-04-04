use bevy::prelude::*;
use spacefight4ever_ui::{
    prelude::{UiElementSize, UiTheme, UiWindowZCounter, ui_window_bundle},
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
    let mut window_id = Entity::PLACEHOLDER;

    // Spawn dialog panel under DialogRoot
    commands
        .entity(parent)
        .with_children(|parent| {
            window_id =
                parent.spawn(
                    ui_window_bundle(
                    "Ship Equipment".into(),
                    UiWindowType::Standard,
                    theme,
                    button_skins,
                    window_skins),
                )
                .id();
        });

    window_id
}