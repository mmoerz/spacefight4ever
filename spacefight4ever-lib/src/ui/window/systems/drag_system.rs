use bevy::{ecs::relationship::Relationship, picking::window, prelude::*};
use bevy::input::mouse::MouseMotion;
use crate::ui::window::component::*;

// fn start_drag(
//     mut commands: Commands,
//     q: Query<(Entity, &Interaction, &ChildOf), (Changed<Interaction>, With<WindowTitleBar>)>,
//     window: Query<Entity, With<UiWindow>>,
//     window_transforms: Query<&GlobalTransform, With<UiWindow>>,
// ) {
//     for (_entity, interaction, parent) in &q {
//         if *interaction != Interaction::Pressed {
//             continue;
//         }

//         if let Ok(parent_entity) = window.get(parent.get()) {
//             if let Ok(transform) = window_transforms.get(parent_entity) {
//                 commands.entity(parent_entity).with_child(
//                     UiWindowDrag {
//                         offset: transform.translation().truncate(),
//                     }
//                 );
//             }
//         }
//     }
// }

// fn drag_windows(
//     mut q: Query<(&mut Node, &WindowDrag), With<UiWindow>>,
//     mut mouse_motion_reader: MessageReader<MouseMotion>,
// ) {
//     for (mut node, drag) in &mut q {
//         for mouse_motion in mouse_motion_reader.read() {
//             node.left = Val::Px(mouse_motion.delta.x +.position.x - drag.offset.x);
//             node.top = Val::Px(mouse_motion.position.y - drag.offset.y);
//         }
//     }
// }

pub fn stop_drag(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    q: Query<Entity, With<UiWindowDrag>>,
) {
    if mouse.just_released(MouseButton::Left) {
        for e in &q {
            commands.entity(e).remove::<UiWindowDrag>();
        }
    }
}