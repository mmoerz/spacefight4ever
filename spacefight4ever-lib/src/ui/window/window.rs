use avian3d::collision::collider::contact_query::TimeOfImpactStatus;
use bevy::prelude::*;
use bevy::ecs::bundle::Bundle;
use serde::de;

use crate::ui::window::bundle::{UiTextBundle, UiWindowBundle};
use crate::ui::window::component::UiWindowTitleBar;
use crate::ui::window::structs::UiElementSize;
use crate::ui::window::consts::{HEIGHT_TITLE_BAR, HEIGHT_STATUS_BAR};


pub fn window_bundle(
    title: &str,
    left: f32, top: f32,
    width: f32, height: f32,
    ui_size: UiElementSize,
    font: Handle<Font>,
    icon_menu: Handle<Image>,
    icon_close: Handle<Image>,
    icon_minimize: Handle<Image>,
    icon_maximize: Handle<Image>,
) -> impl Bundle {
    {(
        UiWindowBundle {
            node: Node {
                width: Val::Px(width),
                height: Val::Px(height),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                position_type: PositionType::Absolute,
                left: Val::Px(left),
                top: Val::Px(top),
                ..default()
            },
            background: BackgroundColor(Color::WHITE),
            ..default()
        },
        children![
            (
                UiWindowTitleBar,
                Node {
                    width: percent(100.),
                    height: px(HEIGHT_TITLE_BAR[ui_size]),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.4, 0.2, 0.2)),
                children![
                    (
                        ImageNode::new(icon_menu),
                    ).with_children(|parent| {
                        parent.spawn((
                            Node {
                                //width: Val::Percent(100.),
                                height: px(HEIGHT_TITLE_BAR[ui_size]),
                                align_self: AlignSelf::Stretch,
                                ..default()
                            },
                            UiTextBundle::new(
                                title,
                                font.clone(),
                                24.0,
                                Color::WHITE
                        )));
                        parent.spawn((
                            Node {
                                width: px(HEIGHT_TITLE_BAR[ui_size]),
                                height: px(HEIGHT_TITLE_BAR[ui_size]),
                                ..default()
                            },
                            ImageNode::new(icon_maximize),
                        ));
                        parent.spawn((
                            Node {
                                width: px(HEIGHT_TITLE_BAR[ui_size]),
                                height: px(HEIGHT_TITLE_BAR[ui_size]),
                                ..default()
                            },
                            ImageNode::new(icon_minimize),
                        ));
                        parent.spawn((
                            Node {
                                width: px(HEIGHT_TITLE_BAR[ui_size]),
                                height: px(HEIGHT_TITLE_BAR[ui_size]),
                                ..default()
                            },
                            ImageNode::new(icon_close),
                        ));
                    })
                ]
            ), (
                Node {
                    width: percent(100.),
                    height: percent(100.),
                    ..default()
                }
            ), (
                Node {
                    width: percent(100.),
                    height: px(HEIGHT_STATUS_BAR[ui_size]),
                    ..default()
                },
            )
        ]
    )}
}