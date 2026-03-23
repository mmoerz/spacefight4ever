use bevy::{ecs::relationship::Relationship, prelude::*};

use crate::ui::window::component::*;
use crate::ui::window::window::get_window_node;

pub fn window_resize_system(
    on_drag: On<Pointer<Drag>>,
    resize_query: Query<(&UiWindowResizeHandle, &ChildOf)>,
    windows: Query<Entity, With<UiWindow>>,
    parents: Query<&ChildOf>,
    mut node_query: Query<&mut Node>,
) {
    // only process if the drag target is a resize handle
    let target = on_drag.event_target();

    if let Ok((handle, child_parent)) = resize_query.get(target) {
        if let Some(window) = get_window_node(windows, child_parent.get(), &parents) {
            if let Ok(mut node) = node_query.get_mut(window) {
                match handle.side {
                    ResizeSide::BottomRight => {
                        let w = match node.width { Val::Px(v) => v, _ => 300.0 };
                        let h = match node.height { Val::Px(v) => v, _ => 200.0 };
                        node.width = Val::Px((w + on_drag.delta.x).max(50.0));
                        node.height = Val::Px((h + on_drag.delta.y).max(50.0));
                    },
                    ResizeSide::Right => {
                        let w = match node.width { Val::Px(v) => v, _ => 300.0 };
                        node.width = Val::Px((w + on_drag.delta.x).max(50.0));
                    },
                    ResizeSide::Bottom => {
                        let h = match node.height { Val::Px(v) => v, _ => 200.0 };
                        node.height = Val::Px((h + on_drag.delta.y).max(50.0));
                    },
                    _ => {}
                }
            }
        }
    }
}