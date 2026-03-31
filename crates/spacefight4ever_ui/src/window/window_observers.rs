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
        if z_query.contains(current) {
            let mut z = z_query.get_mut(current).unwrap();
            z.0 = counter.inc();
            focus.set(current);
            return;
        }

        current = match parents.get(current) {
            Ok(p) => p.get(),
            Err(_) => return,
        };
    }
}

#[derive(Component)]
pub struct UiWindowDragging(Entity);

pub fn on_window_titlebar_drag_start(
    on_drag_start: On<Pointer<DragStart>>,
    parents: Query<&ChildOf, With<UiWindowTitleBar>>,
    mut commands: Commands,
) {
    if let Ok(parent) = parents.get(on_drag_start.event_target()) {
        commands.entity(on_drag_start.event_target())
            .insert(UiWindowDragging(parent.get()));
    }
}

pub fn on_window_titlebar_drag(
    on_drag: On<Pointer<Drag>>,
    mut query: Query<&mut UiTransform, With<UiWindow>>,
    dragging: Query<&UiWindowDragging>,
) {
    if let Ok(drag) = dragging.get(on_drag.event_target()) {
        //println!("name: {:?}", name);
        if let Ok(mut transform) = query.get_mut(drag.0) {
            // Extract the current values as f32
            let Val::Px(x) = transform.translation.x else { return };
            let Val::Px(y) = transform.translation.y else { return };

            transform.translation = Val2::px(
                x + on_drag.delta.x,
                y + on_drag.delta.y
            );
        }
    }
}

pub fn on_window_titlebar_drag_end(
    on_drag_end: On<Pointer<DragEnd>>,
    mut query: Query<(&mut Node, &mut UiTransform)>,
    dragging: Query<&UiWindowDragging>,
    mut commands: Commands,
) {
    if let Ok(drag) = dragging.get(on_drag_end.event_target()) {
        if let Ok((mut node, mut transform)) = query.get_mut(drag.0) {
            let Val::Px(dx) = transform.translation.x else { return };
            let Val::Px(dy) = transform.translation.y else { return };

            if let Val::Px(left) = node.left {
                node.left = Val::Px(left + dx);
            }

            if let Val::Px(top) = node.top {
                node.top = Val::Px(top + dy);
            }
            transform.translation = Val2::ZERO;
        }
        commands.entity(on_drag_end.event_target()).remove::<UiWindowDragging>();
    }
}