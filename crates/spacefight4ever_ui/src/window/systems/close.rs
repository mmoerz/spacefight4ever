use bevy::prelude::*;

use crate::component::*;
use crate::window::window::get_window_node;

pub fn close_windows(
    mut commands: Commands,
    interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<UiWindowCloseButton>)>,
    parents: Query<&ChildOf>,
    windows: Query<Entity, With<UiWindow>>,
) {
    for (button_entity, interaction) in &interaction_query {
        if *interaction == Interaction::Pressed {
            // find the parent window
            if let Some(window_entity) = get_window_node(&windows, button_entity, &parents) {
                // despawn window and all children
                commands.entity(window_entity).despawn();
                println!("Closed window {:?}", window_entity);
            }
        }
    }
}