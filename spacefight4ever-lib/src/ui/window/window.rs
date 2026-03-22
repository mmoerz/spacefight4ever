use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use bevy::ecs::bundle::Bundle;

use crate::ui::window::bundle::{UiTextBundle, UiWindowBundle};
use crate::ui::window::component::UiWindowTitleBar;
use crate::ui::window::structs::UiElementSize;
use crate::ui::window::consts::{HEIGHT_TITLE_BAR, HEIGHT_STATUS_BAR};

pub struct UiWindowPlugin;

impl Plugin for UiWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_window_titelbar_drag_start)
            .add_observer(on_window_titelbar_drag)
            .add_observer(on_window_titelbar_drag_end);
    }
}

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
    let margin1 = UiRect {
        left: px(1.),
        right: px(1.),
        top: px(1.),
        bottom: px(1.),
    };
    let bar_height = HEIGHT_TITLE_BAR[ui_size] -2.;

    {(
        Name::new("AWindow"),
        UiWindowBundle {
            node: Node {
                width: Val::Px(width),
                height: Val::Px(height),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                position_type: PositionType::Absolute,
                left: Val::Px(left),
                top: Val::Px(top),
                ..default()
            },
            background: BackgroundColor(Color::srgb(0., 0.6, 0.75)),
            ..default()
        },
        children![
            (
                Name::new("WindowTitleBar"),
                UiWindowTitleBar,
                Node {
                    width: percent(100.),
                    height: px(HEIGHT_TITLE_BAR[ui_size]),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.4, 0.2, 0.2)),
                Pickable {
                    should_block_lower: true,
                    is_hoverable: true,
                    ..default()
                },
                children![
                    (
                        Node {
                            width: px(HEIGHT_TITLE_BAR[ui_size]),
                            height: px(HEIGHT_TITLE_BAR[ui_size]),
                            margin: margin1,
                            ..default()
                        },
                        ImageNode::new(icon_menu),
                    ), (
                        Node {
                            //width: Val::Percent(100.),
                            height: px(HEIGHT_TITLE_BAR[ui_size]),
                            justify_content: JustifyContent::Stretch,
                            align_self: AlignSelf::Stretch,
                            ..default()
                        },
                        UiTextBundle::new(
                            title,
                            font.clone(),
                            bar_height,
                            Color::WHITE
                        ),
                    ), (
                        Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            margin: UiRect { left: Val::Auto, ..default() },
                            ..default()
                        },
                        children![
                            (
                                Node {
                                    width: px(HEIGHT_TITLE_BAR[ui_size]),
                                    height: px(HEIGHT_TITLE_BAR[ui_size]),
                                    margin: margin1,
                                    ..default()
                                },
                                ImageNode::new(icon_maximize),
                            ), (
                                Node {
                                    width: px(HEIGHT_TITLE_BAR[ui_size]),
                                    height: px(HEIGHT_TITLE_BAR[ui_size]),
                                    margin: margin1,
                                    ..default()
                                },
                                ImageNode::new(icon_minimize),
                            ), (
                                Node {
                                    width: px(HEIGHT_TITLE_BAR[ui_size]),
                                    height: px(HEIGHT_TITLE_BAR[ui_size]),
                                    margin: margin1,
                                    ..default()
                                },
                                ImageNode::new(icon_close),
                            ),
                        ]),
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

fn on_window_titelbar_drag_start(
    on_drag_start: On<Pointer<DragStart>>,
    mut query: Query<(&UiWindowTitleBar ,&mut GlobalZIndex)>,
    parents: Query<&ChildOf>
) {
    if let Ok((_bar, mut node_zindex)) = query.get_mut(on_drag_start.event_target()) {
        node_zindex.0 = 1;

        if let Ok(parent) = parents.get(on_drag_start.event_target()) {
            if let Ok((_bar, mut zindex)) = query.get_mut(parent.get()) {
                zindex.0 = 1;
            }
        }
    }
}

fn on_window_titelbar_drag(
    on_drag: On<Pointer<Drag>>,
    mut query: Query<&mut UiTransform>,
    parents: Query<(&UiWindowTitleBar, &Name, &ChildOf)>
) {
    if let Ok((_bar, name, parent)) = parents.get(on_drag.event_target()) {
        //println!("name: {:?}", name);
        if let Ok(mut transform) = query.get_mut(parent.get()) {
            // Extract the current values as f32
            let current_x = match transform.translation.x {
                Val::Px(v) => v,
                _ => 0.0, // fallback if not px
            };
            let current_y = match transform.translation.y {
                Val::Px(v) => v,
                _ => 0.0,
            };
            let x = current_x + on_drag.delta.x;
            let y = current_y + on_drag.delta.y;
            transform.translation = Val2::px(x, y);
        }
    }
}

fn on_window_titelbar_drag_end(
    on_drag_end: On<Pointer<DragEnd>>,
    mut query: Query<(&mut UiTransform, &mut Outline, &mut GlobalZIndex)>,
    parents: Query<(&UiWindowTitleBar,&ChildOf)>
) {
    if let Ok((_bar, parent)) = parents.get(on_drag_end.event_target()) {
        if let Ok((mut transform, mut outline, mut global_zindex)) = query.get_mut(parent.get()) {
            transform.translation = Val2::ZERO;
            outline.color = Color::NONE;
            global_zindex.0 = 0;
        }
    }
}

// fn on_window_titelbar_drag_drop(
//     on_drag_drop: On<Pointer<DragDrop>>,
//     mut query: Query<&mut Node>,
//     parents: Query<&ChildOf>
// ) {
//     if let Ok(parent) = parents.get(on_drag_drop.event_target()) {
//         if let Ok([mut a, mut b]) = query.get_many_mut([on_drag_drop.event_target(), on_drag_drop.dropped]) {
//             // core::mem::swap(&mut a.grid_row, &mut b.grid_row);
//             // core::mem::swap(&mut a.grid_column, &mut b.grid_column);
//         }
//     }
// }