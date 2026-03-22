use bevy::{ecs::relationship::Relationship, prelude::*};

use crate::ui::window::component::*;


pub fn minimize_windows(
    mut q: Query<(&Interaction, &ChildOf), (Changed<Interaction>, With<UiWindowMaximizeButton>)>,
    //window: Query<Entity, With<UiWindow>>,
    mut windows: Query<(&UiWindow, &mut UiWindowState)>,
) {
    for (interaction, parent) in &mut q {
        if *interaction == Interaction::Pressed {
            if let Ok((_window, mut state)) = windows.get_mut(parent.get()) {
                state.minimized = !state.minimized;
            }
        }
    }
}

pub fn apply_minimize(
    mut q: Query<(&UiWindowState, &mut Node), With<UiWindow>>,
) {
    for (state, mut node) in &mut q {
        if state.minimized {
            node.height = Val::Px(30.);
        }
    }
}

pub fn maximize_windows(
    mut q: Query<(&Interaction, &ChildOf), (Changed<Interaction>, With<UiWindowMaximizeButton>)>,
    mut windows: Query<&mut UiWindowState>,
) {
    for (interaction, parent) in &mut q {
        if *interaction == Interaction::Pressed {
            if let Ok(mut state) = windows.get_mut(parent.get()) {
                state.maximized = !state.maximized;
            }
        }
    }
}

pub fn apply_maximize(
    mut q: Query<(&UiWindowState, &mut Node), With<UiWindow>>,
) {
    for (state, mut node) in &mut q {
        if state.maximized {
            node.left = Val::Px(0.);
            node.top = Val::Px(0.);
            node.width = Val::Percent(100.);
            node.height = Val::Percent(100.);
        }
    }
}