use bevy::prelude::*;

use bevy_ui_widgets::{
    observe, slider_self_update,
};

use crate::ui::overlay::slider::{horizontal_slider, ValueLabel};

const TEXT_COLOR_LESSWHITE: Color = Color::srgb(0.8, 0.8, 0.8);

#[derive(Component)]
pub struct Settings;

pub fn spawn_settings(
    commands: &mut Commands, 
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
                    align_items: AlignItems::Stretch,
                    justify_content: JustifyContent::FlexStart,
                    ..default()
                },
                BackgroundColor(Color::srgba(0., 0.4, 0.4, 0.4)),
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
                        column_gap: px(10.),
                        ..default()
                    },
                )).with_children(|tabrow| {
                    tabrow.spawn((
                        Text::new("Mouse Sensitivity: "),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 15.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR_LESSWHITE),
                    ));
                    let label_id = tabrow
                        .spawn((
                            Text::new("50"),
                            TextFont {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 15.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        ))
                        .id();

                    tabrow.spawn((
                        horizontal_slider(0., 0.05, 0.005),
                        ValueLabel(label_id),
                        observe(slider_self_update),
                    ));
                }
            );
        });
    });
}

fn update_value_labels(
    sliders: Query<(&SliderValue, &ValueLabel), (Changed<SliderValue>, With<UiSlider>)>,
    mut texts: Query<&mut Text>,
) {
    for (value, label) in sliders.iter() {
        if let Ok(mut text) = texts.get_mut(label.0) {
            if value.0 > 0.5 {
                **text = format!("{:.0}", value.0);
            } else {
                **text = format!("{:.3}", value.0);
            }
        }
    }
}