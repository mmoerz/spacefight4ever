use bevy::{ecs::relationship::Relationship, prelude::*};

use crate::ui::window::component::*;

pub fn start_resize(
    mut commands: Commands,
    q: Query<(Entity, &Interaction, &ChildOf), (Changed<Interaction>, With<UiWindowResizeHandle>)>,
    windows: Query<(&Node, &GlobalTransform), With<UiWindow>>,
) {
    for (_, interaction, parent) in &q {
        if *interaction == Interaction::Pressed {
            if let Ok((node, transform)) = windows.get(parent.get()) {
                commands.entity(parent.get()).insert(UiWindowResize {
                    start_size: Vec2::new(300., 200.), // extract from style
                    start_pos: transform.translation().truncate(),
                });
            }
        }
    }
}

// pub fn resize_windows(
//     mut q: Query<(&mut Node, &WindowResize), With<UiWindow>>,
//     cursor: Res<CursorPosition>,
// ) {
//     for (mut node, resize) in &mut q {
//         let new_size = cursor - resize.start_pos;

//         node.width = Val::Px(new_size.x.max(150.));
//         node.height = Val::Px(new_size.y.max(100.));
//     }
// }

