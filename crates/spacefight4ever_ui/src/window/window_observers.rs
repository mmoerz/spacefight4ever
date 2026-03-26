use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;

use crate::component::*;
use crate::resource::*;


pub fn on_window_click_focus(
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

// pub fn on_window_titlebar_drag_start(
//     on_drag_start: On<Pointer<DragStart>>,
//     mut z_query: Query<&mut GlobalZIndex>,
//     parents: Query<&ChildOf, With<UiWindowTitleBar>>,
//     children: Query<&Children>,
//     mut zindex: ResMut<UiWindowZCounter>,
// ) {
//     if let Ok(parent) = parents.get(on_drag_start.event_target()) {

//     }
// }

pub fn on_window_titlebar_drag(
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

pub fn on_window_titlebar_drag_end(
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