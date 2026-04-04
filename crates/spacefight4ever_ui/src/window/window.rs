use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;

use crate::consts::*;
use crate::structs::*;
use crate::bundle::*;
use crate::structs::UiElementSize;
use crate::component::*;

use crate::ui::atlasbutton::*;

use crate::resource::UiWindowZCounter;

use crate::ui::assets::{theme::*, atlasbuttonskin::ButtonSkin, windowsskin::WindowSkin};
use crate::ui::button::{UiButtonType, UiWindowType};

/// Window bundle using UiTheme and UiButtonType
pub fn window_bundle(
    title: &str,
    left: f32,
    top: f32,
    width: f32,
    height: f32,
    ui_size: UiElementSize,
    font: Handle<Font>,
    mut z_index: ResMut<UiWindowZCounter>,
    window_ninepatch_texture: Handle<Image>,
    theme: &UiTheme, // pass theme here
    skins: &Assets<ButtonSkin>, // pass skins here
    window_skins: &Assets<WindowSkin>, // pass skins here
) -> impl Bundle {
    let slicer = TextureSlicer {
        border: BorderRect::all(20.0),
        center_scale_mode: SliceScaleMode::Stretch,
        sides_scale_mode: SliceScaleMode::Stretch,
        max_corner_scale: 1.0,
    };

    let margin1 = UiRect {
        left: Val::Px(1.),
        right: Val::Px(2.),
        top: Val::Px(4.),
        bottom: Val::Px(1.),
    };
    let bar_height = HEIGHT_TITLE_BAR[ui_size] - 2.;

    let window_layout = theme.get_window_skin(UiWindowType::Standard)
        .unwrap();

    let win_l: &WindowSkin = window_skins.get(window_layout).unwrap();

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
                    index: 3,
                    layout: win_l.atlas.clone(),
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
                Pickable {
                    should_block_lower: true,
                    is_hoverable: true,
                    ..default()
                },
                children![
                    // Menu button
                    ui_thematic_button_bundle(UiButtonType::Menu, theme, HEIGHT_TITLE_BAR[ui_size], margin1, skins),
                    // Title text
                    (
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
                            theme.on_surface,
                        ),
                        Visibility::Inherited,
                    ),
                    // Right buttons (minimize, maximize, close)
                    (
                        Node {
                            display: Display::Flex,
                            flex_direction: FlexDirection::Row,
                            margin: UiRect { left: Val::Auto, ..default() },
                            ..default()
                        },
                        Visibility::Inherited,
                        children![
                            ui_thematic_button_bundle(UiButtonType::Minimize, theme, HEIGHT_TITLE_BAR[ui_size], margin1, skins),
                            ui_thematic_button_bundle(UiButtonType::Maximize, theme, HEIGHT_TITLE_BAR[ui_size], margin1, skins),
                            ui_thematic_button_bundle(UiButtonType::Close, theme, HEIGHT_TITLE_BAR[ui_size], margin1, skins),
                        ],
                    )
                ]
            ),
            // Window body + resize handles
            (
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
                    ),
                    (
                        Node {
                            width: px(5.),
                            height: Val::Percent(100.),
                            ..default()
                        },
                        UiWindowResizeHandle { side: ResizeSide::Right },
                    )
                ]
            ),
            (
                Node {
                    width: Val::Percent(100.),
                    height: px(5.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                children![
                    (
                        UiWindowMain,
                        Node {
                            width: Val::Percent(100.),
                            height: px(5.),
                            ..default()
                        },
                        UiWindowResizeHandle { side: ResizeSide::Bottom },
                    ),
                    (
                        Node {
                            width: px(5.),
                            height: px(5.),
                            ..default()
                        },
                        UiWindowResizeHandle { side: ResizeSide::BottomRight },
                    )
                ]
            )
        ]
    )
}

pub fn get_window_node(
    windows: &Query<Entity, With<UiWindow>>,
    mut current: Entity,
    parents: &Query<&ChildOf>,
) -> Option<Entity> {
    while let Ok(parent) = parents.get(current) {
        if windows.contains(current) {
            return Some(current);
        }
        current = parent.get();
    }

    windows.contains(current).then_some(current)
}