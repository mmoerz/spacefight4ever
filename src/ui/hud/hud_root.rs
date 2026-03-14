use bevy::prelude::*;
use crate::ui::{hud::ship_modul_bar::{setup_hex_grid, ship_module_bar}, layers::*};

use super::health_bar::*;

pub fn spawn_hud(
    mut commands: Commands,
    ui_layers: Res<UiLayers>,
    asset_server: Res<AssetServer>,
) {
    commands
        .entity(ui_layers.hud_root)
        .with_children(|parent| {
            parent.spawn(health_bar(&asset_server));
            //parent.spawn(setup_hex_grid(commands, asset_server))
            //parent.spawn(ship_module_bar(&asset_server));
        });

    setup_hex_grid(ui_layers.hud_root, commands, &asset_server)
}