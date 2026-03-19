use bevy::prelude::*;
use crate::ui::bundle::UiButtonBundle;
use crate::ui::dialog_manager::{DialogButton, DialogEntity};

/// Spawns a "Confirm Exit" dialog under the DialogRoot layer
pub fn spawn_confirm_exit_dialog(
    commands: &mut Commands,
    dialog_root: Entity,
    asset_server: &Res<AssetServer>,
) {
    let font_handle = asset_server.load("fonts/FiraSans-Bold.ttf");

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
                Name::new("Dlg_c_exit"),
                Node {
                    width: px(300.0),
                    height: px(200.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    left: px(100.0),
                    top: px(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.99, 0., 0., 0.9)),
                Visibility::Visible,
            ))
            .with_children(|dialog| {
                dialog.spawn((
                    Text::new("Exit Game?"),
                    TextColor::WHITE,
                    TextFont {
                        font: font_handle.clone(),
                        font_size: 30.0,
                        ..default()
                    },
                    Visibility::Inherited,
                ));

                dialog.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(20.0),
                        ..default()
                    },
                    Visibility::Inherited,
                ))
                .with_children(|buttons| {
                    buttons.spawn((
                        UiButtonBundle {
                            ..Default::default()
                        },
                        DialogButton::ConfirmExitYes,
                        children![(
                            Text::new("Yes"),
                            TextColor::BLACK,
                            TextFont {
                                font: font_handle.clone(),
                                font_size: 30.0,
                                ..default()
                            },
                            
                        )]
                    ));
                    buttons.spawn((
                        UiButtonBundle {
                            ..Default::default()
                        },
                        DialogButton::ConfirmExitNo,
                        children![(
                            Text::new("No"),
                            TextColor::BLACK,
                            TextFont {
                                font: font_handle.clone(),
                                font_size: 30.0,
                                ..default()
                            },
                        )]
                    ));
                });
            });
        });
}