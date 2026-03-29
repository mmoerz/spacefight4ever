use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;

use crate::consts::*;
use crate::structs::*;
use crate::bundle::*;
use crate::structs::UiElementSize;
use crate::component::*;

use crate::resource::UiWindowZCounter;

const BUTTON_ATLAS_INDEX_CANCEL: usize = 1;
const BUTTON_ATLAS_INDEX_OK: usize = 0;
const BUTTON_ATLAS_INDEX_MINUS: usize = 3;
const BUTTON_ATLAS_INDEX_PLUS: usize = 2;
const BUTTON_ATLAS_INDEX_MENU: usize = 4;

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
                    index: 11,
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
                //BackgroundColor(Color::srgb(0.4, 0.2, 0.2)),
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
                        UiAtlasButtonIndex(BUTTON_ATLAS_INDEX_MENU),
                        ImageNode::from_atlas_image(
                            button_atlas_texture.clone(),
                            TextureAtlas {
                                index: BUTTON_ATLAS_INDEX_MENU,
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
                                UiAtlasButtonIndex(BUTTON_ATLAS_INDEX_MINUS),
                                ImageNode::from_atlas_image(
                                    button_atlas_texture.clone(),
                                    TextureAtlas {
                                        index: BUTTON_ATLAS_INDEX_MINUS,
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
                                UiAtlasButtonIndex(BUTTON_ATLAS_INDEX_PLUS),
                                ImageNode::from_atlas_image(
                                    button_atlas_texture.clone(),
                                    TextureAtlas {
                                        index: BUTTON_ATLAS_INDEX_PLUS,
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
                                UiAtlasButtonIndex(BUTTON_ATLAS_INDEX_CANCEL),
                                ImageNode::from_atlas_image(
                                    button_atlas_texture.clone(),
                                    TextureAtlas {
                                        index: BUTTON_ATLAS_INDEX_CANCEL,
                                        layout: button_layout.clone(),
                                    },
                                )
                                .with_mode(NodeImageMode::Sliced(button_slicer.clone())),
                            ), 
                        ]),
                ]
            ), (
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
                            width: percent(100.),
                            height: percent(100.),
                            ..default()
                        },
                    ), (
                        Node {
                            width: px(border),
                            height: percent(100.),
                            ..default()
                        },
                        UiWindowResizeHandle { side: ResizeSide::Right },
                        //BackgroundColor(Color::BLACK)
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
                        UiWindowMain,
                        Node {
                            width: percent(100.),
                            height: px(border),
                            ..default()
                        },
                        UiWindowResizeHandle { side: ResizeSide::Bottom },
                        //BackgroundColor(Color::srgb(0., 9., 0.))
                    ), (
                        Node {
                            width: px(border),
                            height: px(border),
                            ..default()
                        },
                        UiWindowResizeHandle { side: ResizeSide::BottomRight },
                        //BackgroundColor(Color::srgb(0., 9., 0.))
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