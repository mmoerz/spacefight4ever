use avian3d::parry::partitioning::BvhLeafCost;
use bevy::ecs::relationship::Relationship;
use bevy::picking::window;
use bevy::prelude::*;
use bevy::ecs::bundle::Bundle;
use bevy::state::commands;

use crate::plugin::ui_window_plugin;
use crate::ui::window::bundle::{UiTextBundle, UiWindowBundle, UiImageButtonBundle};
use crate::ui::window::component::*;
use crate::ui::window::structs::UiElementSize;
use crate::ui::window::consts::{HEIGHT_TITLE_BAR, HEIGHT_STATUS_BAR};

use crate::ui::window::systems::minmax::*;
use crate::ui::window::systems::resize::*;

pub struct UiWindowPlugin;

impl Plugin for UiWindowPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiWindowZCounter>()
            .init_resource::<UiWindowFocused>()
            .init_resource::<UiWindowAtlas>()
            .add_observer(on_window_click_focus)
            .add_observer(on_window_titlebar_drag_start)
            .add_observer(on_window_titlebar_drag)
            .add_observer(on_window_titlebar_drag_end)
            .add_observer(window_resize_system)
            .add_message::<UiWindowsStatusChangeRequest>()
            .add_systems(Startup, setup_window_bundle)
            .add_systems(Update, minimize_windows)
            .add_systems(Update, apply_window_status_change)
            .add_systems(Update, maximize_windows);
        app.add_systems(Update, window_button_interaction_system);
    }
}

/// register texture atlas?
pub fn setup_window_bundle(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let atlas_layout =
        TextureAtlasLayout::from_grid(UVec2::new(50, 50), 4, 4, Some(UVec2::splat(2)), None);
    let window_atlas_handle = texture_atlases.add(atlas_layout);
    let atlas_layout =
        TextureAtlasLayout::from_grid(UVec2::new(54, 54), 20, 3, Some(UVec2::splat(2)), None);
    let button_atlas_handle = texture_atlases.add(atlas_layout);

    commands.insert_resource(UiWindowAtlas { 
        window_layout: window_atlas_handle,
        button_layout: button_atlas_handle 
    });
}


pub fn window_bundle(
    title: &str,
    left: f32, top: f32,
    width: f32, height: f32,
    ui_size: UiElementSize,
    font: Handle<Font>,
    window_ninepatch_texture: Handle<Image>,
    button_atlas_texture: Handle<Image>,
    window_layout: Handle<TextureAtlasLayout>,
    button_layout: Handle<TextureAtlasLayout>,
) -> impl Bundle {
    let slicer = TextureSlicer {
        border: BorderRect::all(24.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };
    let button_slicer = TextureSlicer {
        border: BorderRect::all(24.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    let border: f32 = 5.;
    let margin1 = UiRect {
        left: px(1.),
        right: px(1.),
        top: px(1.),
        bottom: px(1.),
    };
    let bar_height = HEIGHT_TITLE_BAR[ui_size] -2.;

    {(
        Name::new("Window"),
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
            //background: BackgroundColor(Color::srgb(0., 0.6, 0.75)),
            image_node: ImageNode::from_atlas_image(
                window_ninepatch_texture,
                TextureAtlas {
                    index: 0,
                    layout: window_layout.clone(),
                },
            )
            .with_mode(NodeImageMode::Sliced(slicer.clone())),
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
                        UiWindowMenuButton,
                        UiImageButtonBundle {
                            button: Button,
                            node: Node {
                                width: px(HEIGHT_TITLE_BAR[ui_size]),
                                height: px(HEIGHT_TITLE_BAR[ui_size]),
                                display: Display::Block,
                                margin: margin1,
                                ..default()
                            },
                            state: UiImageButtonState::Normal,
                            ..default()
                        },
                        ImageNode::from_atlas_image(
                            button_atlas_texture.clone(),
                            TextureAtlas {
                                index: 3,
                                layout: button_layout.clone(),
                            },
                        )
                        .with_mode(NodeImageMode::Sliced(button_slicer.clone())),
                        Visibility::Visible,
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
                                UiWindowMinimizeButton,
                                UiImageButtonBundle {
                                    button: Button,
                                    node: Node {
                                        width: px(HEIGHT_TITLE_BAR[ui_size]),
                                        height: px(HEIGHT_TITLE_BAR[ui_size]),
                                        margin: margin1,
                                        ..default()
                                    },
                                    state: UiImageButtonState::Normal,
                                    ..default()
                                },
                                // minimize button
                                ImageNode::from_atlas_image(
                                    button_atlas_texture.clone(),
                                    TextureAtlas {
                                        index: 2,
                                        layout: button_layout.clone(),
                                    },
                                )
                                .with_mode(NodeImageMode::Sliced(button_slicer.clone())),
                                Visibility::Visible,
                            ), (
                                UiWindowMaximizeButton,
                                UiImageButtonBundle {
                                    button: Button,
                                    node: Node {
                                        width: px(HEIGHT_TITLE_BAR[ui_size]),
                                        height: px(HEIGHT_TITLE_BAR[ui_size]),
                                        margin: margin1,
                                        ..default()
                                    },
                                    state: UiImageButtonState::Normal,
                                    ..default()
                                },
                                // maximize button
                                ImageNode::from_atlas_image(
                                    button_atlas_texture.clone(),
                                    TextureAtlas {
                                        index: 1,
                                        layout: button_layout.clone(),
                                    },
                                )
                                .with_mode(NodeImageMode::Sliced(button_slicer.clone())),
                                Visibility::Visible,
                            ), (
                                UiWindowCloseButton,
                                UiImageButtonBundle {
                                    button: Button,
                                    node: Node {
                                        width: px(HEIGHT_TITLE_BAR[ui_size]),
                                        height: px(HEIGHT_TITLE_BAR[ui_size]),
                                        margin: margin1,
                                        ..default()
                                    },
                                    state: UiImageButtonState::Normal,
                                    ..default()
                                },
                                // close button
                                ImageNode::from_atlas_image(
                                    button_atlas_texture.clone(),
                                    TextureAtlas {
                                        index: 0,
                                        layout: button_layout.clone(),
                                    },
                                )
                                .with_mode(NodeImageMode::Sliced(button_slicer.clone())),
                                Visibility::Visible,
                            ), 
                        ]),
                ]
            ), (
                // main body and right side border
                Node {
                    width: percent(100.),
                    height: percent(100.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                children![
                    (
                        UiWindowMain,
                        Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        //BackgroundColor(Color::srgb(0.9,0., 0.))

                    ), (
                        // and the right side for resize
                        Node {
                            width: px(border),
                            height: percent(100.),
                            ..default()
                        },
                        UiWindowResizeHandle { side: ResizeSide::Right },
                        BackgroundColor(Color::BLACK)
                    )],
            ), (
                Node {
                    width: percent(100.),
                    height: px(border),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                children![
                    (
                        Node {
                            width: Val::Percent(100.),
                            ..default()
                        },
                        UiWindowResizeHandle { side: ResizeSide::Bottom },
                        //BackgroundColor(Color::BLACK)
                    ), (
                        Node {
                            height: px(border),
                            width: px(border),
                            ..default()
                        },
                        UiWindowResizeHandle { side: ResizeSide::BottomRight },
                        BackgroundColor(Color::srgb(0.8, 0., 0.8))
                    )
                ],
            )
        ]
    )}
}



/// helper to the the parent/grandparent/... window entity
pub fn get_window_node(
    mut windows: Query<Entity, With<UiWindow>>,
    mut current: Entity,
    parents: &Query<&ChildOf>,
) -> Option<Entity> {
    loop {
        if let Ok(window) = windows.get_mut(current) {
            return Some(window);
        }
        if let Ok(parent) = parents.get(current) {
            current = parent.get();
        } else {
            return None;
        }
    }
}

fn on_window_click_focus(
    on_down: On<Pointer<Press>>,
    mut z_query: Query<&mut GlobalZIndex, With<UiWindow>>,
    mut counter: ResMut<UiWindowZCounter>,
    parents: Query<&ChildOf>,
    mut focus: ResMut<UiWindowFocused>,
) {
    let mut current = on_down.event_target();

    // climb hierarchy until we find UiWindow
    loop {
        if let Ok(mut z) = z_query.get_mut(current) {
            z.0 = counter.inc();
            focus.set(current);
            break;
        }

        if let Ok(parent) = parents.get(current) {
            current = parent.get();
        } else {
            break;
        }
    }
}

fn on_window_titlebar_drag_start(
    on_drag_start: On<Pointer<DragStart>>,
    mut z_query: Query<&mut GlobalZIndex>,
    parents: Query<&ChildOf, With<UiWindowTitleBar>>,
    children: Query<&Children>,
    mut zindex: ResMut<UiWindowZCounter>,
) {
    if let Ok(parent) = parents.get(on_drag_start.event_target()) {

    }
}

fn on_window_titlebar_drag(
    on_drag: On<Pointer<Drag>>,
    mut query: Query<&mut UiTransform>,
    parents: Query<(&UiWindowTitleBar, &ChildOf)>
) {
    if let Ok((_bar, parent)) = parents.get(on_drag.event_target()) {
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

fn on_window_titlebar_drag_end(
    on_drag_end: On<Pointer<DragEnd>>,
    mut query: Query<(&mut Node, &mut UiTransform)>,
    parents: Query<(&UiWindowTitleBar,&ChildOf)>
) {
    if let Ok((_bar, parent)) = parents.get(on_drag_end.event_target()) {
        if let Ok((mut node, mut transform)) = query.get_mut(parent.get()) {
            let dx = match transform.translation.x { Val::Px(v) => v, _ => 0.0 };
            let dy = match transform.translation.y { Val::Px(v) => v, _ => 0.0 };

            if let Val::Px(left) = node.left {
                node.left = Val::Px(left + dx);
            }

            if let Val::Px(top) = node.top {
                node.top = Val::Px(top + dy);
            }
            transform.translation = Val2::ZERO;
        }
    }
}

pub fn window_button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &Button, &mut ImageNode),
        (Changed<Interaction>, Or<(
            With<UiWindowMenuButton>,
            With<UiWindowMinimizeButton>,
            With<UiWindowMaximizeButton>,
            With<UiWindowCloseButton>
        )>)>,
    mut vis_query: Query<(&UiImageButtonState, &mut Visibility)>,
){
    for (interaction, _button, mut image_node  ) in &mut interaction_query {
        if let Some(atlas) = &mut image_node.texture_atlas {
            atlas.index = match *interaction {
                Interaction::Pressed => atlas.index % 20,
                Interaction::Hovered => atlas.index % 20 + 20,
                Interaction::None => atlas.index % 20,
            };
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