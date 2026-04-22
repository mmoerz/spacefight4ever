use bevy::{prelude::*};
use crate::ui::{self, overlay::settings::UiSettingsOpened};
use crate::ui::layers::UiLayers;
use spacefight4ever_ui::{
    prelude::*,
    ui::assets::{atlasbuttonskin::ButtonSkin, windowsskin::WindowSkin},
};
use crate::config::environment::AppConfig;
use spacefight4ever_ui::ui::assets::{theme::UiTheme, assets::UiResources};
use crate::ui::overlay::settings::spawn_settings;

pub struct UiPlugin;

pub fn trigger_exit_dialog(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut events: MessageWriter<ui::messages::DialogRequest>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        events.write(ui::messages::DialogRequest::ConfirmExit);
    }
}

pub fn trigger_ship_equipment_dialog(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    ui_layers: Res<UiLayers>,
    //asset_server: Res<AssetServer>,
    z_index: ResMut<UiWindowZCounter>,
    //mut events: MessageWriter<UiWindowsSwitchAtlasRequest>,
    if_theme: If<Res<UiResources>>,
    themes: Res<Assets<UiTheme>>,
    button_skins: Res<Assets<ButtonSkin>>, // pass skins here
    window_skins: Res<Assets<WindowSkin>>, // pass skins here
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        let theme = &if_theme;
        //events.write(ui::messages::DialogRequest::ShipEquipment);
        ui::dialogs::ship_equipment::spawn_ship_equipment_dialog(
            &mut commands,
            ui_layers.window_root,
            //&asset_server,
            z_index,
            theme,
            &themes,
            &button_skins,
            &window_skins,
        );
    }
    if keyboard.just_pressed(KeyCode::KeyW) {
        //events.write(UiWindowsSwitchAtlasRequest(window_atlas_status.current + 1));
    }
}

pub fn trigger_settings(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    ui_layers: Res<UiLayers>,
    asset_server: Res<AssetServer>,
    config: Res<AppConfig>,
    mut opened: ResMut<UiSettingsOpened>,
) {
    if keyboard.just_pressed(KeyCode::KeyS) {
        if ! opened.opened {
            opened.entity =
                spawn_settings(
                    &mut commands,
                    ui_layers.window_root,
                    &asset_server,
                    &config,
                );
            opened.opened = true;
            //println!("Opened settings {:?}", opened.entity)
        } else {
            //println!("Closing settings {:?}", opened.entity);
            commands.entity(opened.entity).despawn();
            opened.entity = Entity::PLACEHOLDER;
            opened.opened = false;
        }
    }
}

// pub fn move_forward(
//     keyboard: Res<ButtonInput<KeyCode>>,
//     query: Query<(Entity,  
// )