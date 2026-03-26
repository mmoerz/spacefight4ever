use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;

use crate::consts::*;
use crate::structs::*;
use crate::bundle::*;
use crate::structs::UiElementSize;
use crate::component::*;

use crate::resource::UiWindowZCounter;

pub fn window_bundle(
    title: &str,
    left: f32, top: f32,
    width: f32, height: f32,
    ui_size: UiElementSize,
    font: Handle<Font>,
    mut z_index: ResMut<UiWindowZCounter>,
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
        left: Val::Px(1.),
        right: Val::Px(1.),
        top: Val::Px(1.),
        bottom: Val::Px(1.),
    };
    let bar_height = HEIGHT_TITLE_BAR[ui_size] - 2.;

    (
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
            image_node: ImageNode::from_atlas_image(
                window_ninepatch_texture,
                TextureAtlas {
                    index: 0,
                    layout: window_layout.clone(),
                },
            )
            .with_mode(NodeImageMode::Sliced(slicer.clone())),
            z_index: GlobalZIndex(z_index.inc()),
            ..default()
        },
        children![
            (
                Name::new("WindowTitleBar"),
                UiWindowTitleBar,
                Node {
                    width: Val::Percent(100.),
                    height: Val::Px(HEIGHT_TITLE_BAR[ui_size]),
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
                                width: Val::Px(HEIGHT_TITLE_BAR[ui_size]),
                                height: Val::Px(HEIGHT_TITLE_BAR[ui_size]),
                                display: Display::Block,
                                margin: margin1,
                                ..default()
                            },
                            ..default()
                        },
                        UiAtlasButtonIndex(3),
                        ImageNode::from_atlas_image(
                            button_atlas_texture.clone(),
                            TextureAtlas {
                                index: 3,
                                layout: button_layout.clone(),
                            },
                        )
                        .with_mode(NodeImageMode::Sliced(button_slicer.clone())),
                    ), (
                        Node {
                            height: Val::Px(HEIGHT_TITLE_BAR[ui_size]),
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
                                        width: Val::Px(HEIGHT_TITLE_BAR[ui_size]),
                                        height: Val::Px(HEIGHT_TITLE_BAR[ui_size]),
                                        margin: margin1,
                                        ..default()
                                    },
                                    ..default()
                                },
                                UiAtlasButtonIndex(2),
                                ImageNode::from_atlas_image(
                                    button_atlas_texture.clone(),
                                    TextureAtlas {
                                        index: 2,
                                        layout: button_layout.clone(),
                                    },
                                )
                                .with_mode(NodeImageMode::Sliced(button_slicer.clone())),
                            ), (
                                UiWindowMaximizeButton,
                                UiImageButtonBundle {
                                    button: Button,
                                    node: Node {
                                        width: Val::Px(HEIGHT_TITLE_BAR[ui_size]),
                                        height: Val::Px(HEIGHT_TITLE_BAR[ui_size]),
                                        margin: margin1,
                                        ..default()
                                    },
                                    ..default()
                                },
                                UiAtlasButtonIndex(1),
                                ImageNode::from_atlas_image(
                                    button_atlas_texture.clone(),
                                    TextureAtlas {
                                        index: 1,
                                        layout: button_layout.clone(),
                                    },
                                )
                                .with_mode(NodeImageMode::Sliced(button_slicer.clone())),
                            ), (
                                UiWindowCloseButton,
                                UiImageButtonBundle {
                                    button: Button,
                                    node: Node {
                                        width: Val::Px(HEIGHT_TITLE_BAR[ui_size]),
                                        height: Val::Px(HEIGHT_TITLE_BAR[ui_size]),
                                        margin: margin1,
                                        ..default()
                                    },
                                    ..default()
                                },
                                UiAtlasButtonIndex(0),
                                ImageNode::from_atlas_image(
                                    button_atlas_texture.clone(),
                                    TextureAtlas {
                                        index: 0,
                                        layout: button_layout.clone(),
                                    },
                                )
                                .with_mode(NodeImageMode::Sliced(button_slicer.clone())),
                            ), 
                        ]),
                ]
            ), (
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
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
                    ), (
                        Node {
                            width: Val::Px(border),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        UiWindowResizeHandle { side: ResizeSide::Right },
                        BackgroundColor(Color::BLACK)
                    )],
            )
        ]
    )
}

pub fn get_window_node(
    windows: &Query<Entity, With<UiWindow>>,
    mut current: Entity,
    parents: &Query<&ChildOf>,
) -> Option<Entity> {
    loop {
        if let Ok(window) = windows.get(current) {
            return Some(window);
        }
        if let Ok(parent) = parents.get(current) {
            current = parent.get();
        } else {
            return None;
        }
    }
}