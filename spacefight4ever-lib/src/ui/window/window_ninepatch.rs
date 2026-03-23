use bevy::{ecs::relationship::Relationship, prelude::*};

use crate::ui::window::component::{UiWindow, UiWindowNinePatch, UiWindowTitleBar, UiWindowCloseButton, UiWindowResizeHandle, ResizeSide, UiWindowFocused, UiWindowZCounter};

pub struct UiWindowNinePlugin;

impl Plugin for UiWindowNinePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiWindowZCounter>()
            .init_resource::<UiWindowFocused>()
            .add_observer(resize_window_system)
            .add_observer(drag_window_system);
    }
}            


fn sync_window_background(
    windows: Query<(&Node, &Children), With<UiWindow>>,
    mut sprites: Query<&mut Sprite, With<UiWindowBackground>>,
) {
    for (node, children) in &windows {
        let width = match node.width { Val::Px(v) => v, _ => continue };
        let height = match node.height { Val::Px(v) => v, _ => continue };

        for child in children {
            if let Ok(mut sprite) = sprites.get_mut(*child) {
                sprite.custom_size = Some(Vec2::new(width, height));
            }
        }
    }
}


pub fn window_background(texture_handle: Handle<Image>, position: Vec3, size: Vec2, slice_border: f32) -> impl Bundle{
    (
        Sprite {
            image: texture_handle.clone(),
            custom_size: Some(size),
            image_mode: SpriteImageMode::Sliced(TextureSlicer {
                border: BorderRect::all(slice_border),
                center_scale_mode: SliceScaleMode::Tile { stretch_value: 0.1 },
                sides_scale_mode: SliceScaleMode::Tile { stretch_value: 0.2 },
                max_corner_scale: 0.2,
            }),
            ..default()
        },
        Transform::from_translation(position),
        children![(

        )]
    )
    // and spawn the rest of the UI elements on top of it
}

pub fn create_ui_ninepatch_window(
    left: Val, top: Val,
    width: Val, height: Val,
    window_texture: Handle<Image>,
) -> impl Bundle {
    {(
        UiWindow,
        UiWindowNinePatch {
            texture: window_texture.clone(),
            slice: Vec4::new(10.0, 10.0, 10.0, 10.0),
        },
        Node {
            width: width,
            height: height,
            position_type: PositionType::Absolute,
            top: top,
            left: left,
            ..default()
        },
        BackgroundColor(Color::WHITE),
        children![
            (
                // Title bar
                UiWindowTitleBar,
                Node {
                    width: percent(100.),
                    height: px(30.),
                    display: Display::Flex,
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            ), (
                // Close button
                UiWindowCloseButton,
                Node {
                    width: px(20.),
                    height: px(20.),
                    position_type: PositionType::Absolute,
                    right: px(5.),
                    top: px(5.),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.8, 0.1, 0.1)),
            ), (
                // Resize handle (bottom-right)
                UiWindowResizeHandle { side: ResizeSide::BottomRight },
                Node {
                    width: px(15.),
                    height: px(15.),
                    position_type: PositionType::Absolute,
                    right: px(5.),
                    bottom: px(5.),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.2, 0.8, 0.2)),
            )
        ],
    )}
}

/// Simple drag system for windows
pub fn drag_window_system(
    drag: On<Pointer<Drag>>,
    mut query: Query<&mut Node, With<UiWindow>>,
    titlebars: Query<&ChildOf, With<UiWindowTitleBar>>,
) {
    for titlebar_parent in titlebars.iter() {
        if let Ok(mut node) = query.get_mut(titlebar_parent.get()) {
            node.left = Val::Px(
                match node.left {
                    Val::Px(v) => v + drag.delta.x,
                    _ => drag.delta.x,
                }
            );
            node.top = Val::Px(
                match node.top {
                    Val::Px(v) => v + drag.delta.y,
                    _ => drag.delta.y,
                }
            );
        }
    }
}

/// Simple resize system
pub fn resize_window_system(
    drag: On<Pointer<Drag>>,
    mut query: Query<&mut Node, With<UiWindow>>,
    handles: Query<&ChildOf, With<UiWindowResizeHandle>>,
) {
    for handle_parent in handles.iter() {
        if let Ok(mut node) = query.get_mut(handle_parent.get()) {
            node.width = Val::Px(
                match node.width {
                    Val::Px(v) => (v + drag.delta.x).max(50.0),
                    _ => 50.0,
                }
            );
            node.height = Val::Px(
                match node.height {
                    Val::Px(v) => (v + drag.delta.y).max(50.0),
                    _ => 50.0,
                }
            );
        }
    }
}   