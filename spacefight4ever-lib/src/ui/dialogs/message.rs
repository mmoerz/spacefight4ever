use bevy::prelude::*;
use crate::ui::dialog_manager::DialogEntity;
use crate::ui::bundle::*;
use crate::ui::window::bundle::UiTextBundle;

pub fn spawn_message_dialog(
    commands: &mut Commands,
    dialog_root: Entity,
    asset_server: &Res<AssetServer>,
    msg: &String
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // Root overlay for this dialog
    commands
        .entity(dialog_root)
        .with_children(|parent| {
            parent.spawn((
                DialogEntity,
                Node {
                    width: px(300.0),
                    height: px(200.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceEvenly,
                    align_items: AlignItems::Center,
                    left: px(400.0),
                    bottom: px(100.0),
                    ..default()
                },
                BackgroundColor(
                    Color::srgba(0.0, 0.0, 0.0, 0.6)
                ),
            ))
            .with_children(|dialog| {
                // Message text
                dialog.spawn(UiTextBundle::new(
                    msg,
                    font.clone(),
                    24.0,
                    Color::WHITE,
                ));

                // Button row container
                dialog.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(20.0),
                        ..default()
                    },
                    BackgroundColor(Color::NONE),
                ))
                .with_children(|buttons| {
                    // Spawn the OK button
                    // spawn_button(buttons, "OK", font.clone(),
                    // Color::WHITE);
                    buttons.spawn((
                        UiButtonBundle::new(
                            px(120),
                            px(45),
                            Color::srgb(0.3, 0.3, 0.3)
                        ),
                        children![(
                            Text::new("OK"),
                            TextFont{
                                font: font,
                                font_size: 30.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ));
                });
            });
        });
}