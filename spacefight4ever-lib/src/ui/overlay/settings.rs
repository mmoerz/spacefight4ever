use bevy::prelude::*;

use bevy_ui_widgets::{
    observe, slider_self_update, SliderValue
};

use crate::config::environment::AppConfig;
use crate::ui::overlay::slider::{horizontal_slider, ValueLabel};

const TEXT_COLOR_LESSWHITE: Color = Color::srgb(0.8, 0.8, 0.8);

#[derive(Component)]
struct CameraSensitivitySlider;

#[derive(Component)]
pub struct Settings;

pub fn spawn_settings(
    commands: &mut Commands, 
    parent: Entity,
    asset_server: &Res<AssetServer>,
    config: &Res<AppConfig>,
) -> Entity {
    let mut result = Entity::PLACEHOLDER;

    commands.entity(parent)
        .with_children(|top| {
            result = top.spawn((
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
                        Node {
                            width: px(160.0),
                            ..default()
                        },
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
                            Node {
                                width: px(45.0),
                                ..default()
                            },
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
                        horizontal_slider(0., 0.005, config.mouse.sensitivity),
                        ValueLabel(label_id),
                        CameraSensitivitySlider,
                        observe(slider_self_update),
                    ));
                }
            );
        }).id();
    });

    result
}

fn update_value_labels(
    sliders: Query<&SliderValue, (Changed<SliderValue>, With<CameraSensitivitySlider>)>,
    mut config: ResMut<AppConfig>,
) {
    if let Ok(value) = sliders.single() {
        config.mouse.sensitivity = value.0;
    }
}

#[derive(Resource)]
pub struct UiSettingsOpened {
    pub entity: Entity,
    pub opened: bool,
}

impl Default for UiSettingsOpened {
    fn default() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            opened: false,
        }
    }
}

pub struct UiSettingsPlugin;

impl Plugin for UiSettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiSettingsOpened>()
            .add_systems(Update, update_value_labels)
        ;
    }
}