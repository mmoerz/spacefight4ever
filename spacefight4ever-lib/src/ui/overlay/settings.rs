use bevy::prelude::*;

const TEXT_COLOR_LESSWHITE: Color = Color::srgb(0.8, 0.8, 0.8);

#[derive(Component)]
pub struct Settings;

pub fn spawn_settings(
    mut commands: Commands, 
    parent: Entity,
    asset_server: &Res<AssetServer>,
) {
    commands.entity(parent)
        .with_children(|top| {
            top.spawn((
                Node {
                    width: percent(50.0),
                    height: percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            )
        ).with_children(|parent| {
            parent.spawn(
                (
                    Node {
                        width: percent(100.0),
                        height: px(15.0),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                )).with_children(|tabparent| {
                    tabparent.spawn((
                        Node {
                            width: percent(50.0),
                            height: px(15.0),
                            ..default()
                        },
                        Text::new("Settings"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR_LESSWHITE),
                    ));
                });
            parent.spawn(
                (
                    Node {
                        width: percent(100.0),
                        height: px(15.0),
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },
                )).with_children(|tabrow| {
                    tabrow.spawn((
                        Text::new("Mouse Sensitivity"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR_LESSWHITE),
                    ));
                    tabrow.spawn((
                        Node {
                            width: percent(100.0),
                            height: px(15.0),
                            ..default()
                        },
                        
                    ));
                }
            );
        });
    }).id();
}