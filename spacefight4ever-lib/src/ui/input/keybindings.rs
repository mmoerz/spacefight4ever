use bevy::prelude::*;
use crate::ui;
use crate::ui::layers::UiLayers;
use spacefight4ever_ui::prelude::{UiWindowAtlas, UiWindowZCounter};
use spacefight4ever_ui::window::systems::window_atlas_switch::{UiWindowsSwitchAtlasRequest, UiWindowAtlasStatus};

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
    window_atlas: Res<UiWindowAtlas>,
    z_index: ResMut<UiWindowZCounter>,
    mut events: MessageWriter<UiWindowsSwitchAtlasRequest>,
    window_atlas_status: Res<UiWindowAtlasStatus>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        //events.write(ui::messages::DialogRequest::ShipEquipment);
        ui::dialogs::ship_equipment::spawn_ship_equipment_dialog(
            &mut commands,
            ui_layers.window_root,
            &asset_server,
            z_index,
            &window_atlas,
        );
    }
    if keyboard.just_pressed(KeyCode::KeyW) {
        events.write(UiWindowsSwitchAtlasRequest(window_atlas_status.current + 1));
    }
}