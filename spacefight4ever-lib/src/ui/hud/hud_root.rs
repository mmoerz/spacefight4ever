use bevy::prelude::*;
use crate::ui::{hud::ship_modul_bar::setup_hex_grid, layers::*};

use super::health_display::*;

pub fn spawn_hud(
    mut commands: Commands,
    ui_layers: Res<UiLayers>,
    asset_server: Res<AssetServer>,
) {
    let hex_height = 40.0_f32;

    let bottom_hud = 
        commands.spawn((
                Name::new("BottomHud"),
                Node {
                    //width: percent(100.0),       // full width
                    height: px(hex_height * 3.0),           // enough height for grid
                    position_type: PositionType::Absolute,
                    bottom: px(0.0),             // anchor to bottom
                    left: px(0.0),
                    align_self: AlignSelf::Stretch,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                //BackgroundColor(Color::srgb(0.0, 0.7, 0.0))
            )).id();

    commands
        .entity(ui_layers.hud_root)
        .add_child(bottom_hud);
        // .with_children(|hroot| {
        //     hroot.spawn((
        //         Node {
        //             height: px(100),
        //             width: px(100),
        //             align_self: AlignSelf::Auto,
        //             top: px(200),
        //             left: px(200),
        //             ..default()
        //         },
        //         BackgroundColor(Color::BLACK),
        //     ));
        // });

    commands
        .entity(bottom_hud)
        .with_children(|bottom| {
            bottom.spawn((
                // container to offset in the row the rest
                Node {
                    width: px(400),
                    height: px(20),
                    ..default()
                },
                //BackgroundColor(Color::WHITE),
            ));
            
        });
    
    health_display(bottom_hud, &mut commands, &asset_server);
    setup_hex_grid(bottom_hud, &mut commands, &asset_server);
}