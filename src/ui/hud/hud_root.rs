use bevy::prelude::*;
use crate::ui::{hud::ship_modul_bar::{setup_hex_grid, ship_module_bar}, layers::*};

use super::health_bar::*;

pub fn spawn_hud(
    mut commands: Commands,
    ui_layers: Res<UiLayers>,
    asset_server: Res<AssetServer>,
) {
    let hex_size = 40;

    let bottom_hud = 
        commands.spawn((
                Name::new("BottomHud"),
                Node {
                    width: percent(100.0),       // full width
                    height: px(100.0),           // enough height for grid
                    position_type: PositionType::Absolute,
                    bottom: px(0.0),             // anchor to bottom
                    left: px(0.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::FlexEnd,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.0, 0.7, 0.0))
            )).id();

    commands
        .entity(ui_layers.hud_root)
        .add_child(bottom_hud)
        .with_children(|parent| {
            parent.spawn(health_bar(&asset_server));
        });
                    
            //parent.spawn(setup_hex_grid(commands, asset_server))
            //parent.spawn(ship_module_bar(&asset_server));

    setup_hex_grid(bottom_hud, commands, &asset_server)
}