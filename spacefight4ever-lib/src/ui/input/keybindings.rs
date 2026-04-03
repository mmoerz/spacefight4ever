use bevy::prelude::*;
use crate::ui;
use crate::ui::layers::UiLayers;
use spacefight4ever_ui::{
    prelude::{UiWindowZCounter},
    ui::assets::{atlasbuttonskin::ButtonSkin, windowsskin::WindowSkin},
};
use spacefight4ever_ui::window::systems::window_atlas_switch::{UiWindowsSwitchAtlasRequest, UiWindowAtlasStatus};
use spacefight4ever_ui::ui::assets::{theme::UiTheme, assets::UiResources};

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
    asset_server: Res<AssetServer>,
    z_index: ResMut<UiWindowZCounter>,
    //mut events: MessageWriter<UiWindowsSwitchAtlasRequest>,
    if_theme: If<Res<UiResources>>,
    skins: Res<Assets<ButtonSkin>>, // pass skins here
    window_skins: Res<Assets<WindowSkin>>, // pass skins here
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        let theme = &if_theme;
        //events.write(ui::messages::DialogRequest::ShipEquipment);
        ui::dialogs::ship_equipment::spawn_ship_equipment_dialog(
            &mut commands,
            ui_layers.window_root,
            &asset_server,
            z_index,
            theme,
            &skins,
            &window_skins,
        );
    }
    if keyboard.just_pressed(KeyCode::KeyW) {
        //events.write(UiWindowsSwitchAtlasRequest(window_atlas_status.current + 1));
    }
}