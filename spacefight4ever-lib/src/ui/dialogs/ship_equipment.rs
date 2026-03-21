use bevy::prelude::*;
use crate::ui::bundle::UiButtonBundle;
use crate::ui::dialog_manager::{DialogButton, DialogEntity};

/// Spawns a "Confirm Exit" dialog under the DialogRoot layer
pub fn spawn_ship_equipment_dialog(
    commands: &mut Commands,
    dialog_root: Entity,
    asset_server: &Res<AssetServer>,
) {
    let font_handle: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");

    match asset_server.get_load_state(&font_handle) {
        Some(bevy::asset::LoadState::Loaded) => println!("Loaded"),
        Some(bevy::asset::LoadState::Failed(e)) => println!("FAILED: {:?}", e),
        _ => println!("Loading..."),
    }

    // Spawn dialog panel under DialogRoot
    commands
        .entity(dialog_root)
        .with_children(|parent| {
            parent.spawn((
                DialogEntity,
                Name::new("Ship_Equiment_Dialog"),
                Node {
                    width: px(600.0),
                    height: px(400.0),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    left: px(100.0),
                    top: px(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.99, 0., 0., 0.9)),
                Visibility::Visible,
            
                children![
                    (
                        Node {
                            width: px(15),
                            height: percent(100),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0., 0.5, 0.5))
                    ), (
                        Node {
                            width: px(20),
                            height: percent(100),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0., 0.2, 0.5))
                    ), (
                        Node {
                            width: px(240),
                            height: percent(100),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0., 0.5, 0.5))
                    ), (
                        Node {
                            width: px(20),
                            height: percent(100),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0., 0.2, 0.5))
                    ), (
                        Node {
                            width: px(30),
                            height: percent(100),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0., 0.5, 0.5))
                    )
                ]
            ));
            // .with_children(|dialog| {
            //     dialog.spawn((
            //         Node {
            //             width: px(20),
            //             height: percent(100),
            //             ..default()
            //         },
            //         BackgroundColor(Color::srgb(0., 0.5, 0.5))
            //     ));
            // });
        })
        ;

}