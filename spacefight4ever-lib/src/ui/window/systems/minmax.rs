use std::os::linux::raw::stat;

use bevy::{ecs::relationship::Relationship, prelude::*};

use crate::ui::window::component::*;
use crate::ui::window::consts::HEIGHT_TITLE_BAR;

#[derive(Message)]
pub struct UiWindowsStatusChangeRequest {
    window: Entity,
    status: UiWindowStatus,
}

pub fn minimize_windows(
    mut q: Query<(&Interaction, &ChildOf), (Changed<Interaction>, With<UiWindowMinimizeButton>)>,
    mut windows: Query<Entity, With<UiWindow>>,
    mut change_message: MessageWriter<UiWindowsStatusChangeRequest>,
) {
    for (interaction, parent_titelbar) in &mut q {
        if *interaction == Interaction::Pressed {
            if let Ok(window) = windows.get_mut(parent_titelbar.get()) {
                change_message.write(UiWindowsStatusChangeRequest {
                    window: window,
                    status: UiWindowStatus::Minimized,
                });
            }
        }
    }
}

fn save_window_size(node: &Node) -> UiRect {
    UiRect::new( 
        node.left,
        node.width,
        node.top,
        node.height
    )
}

fn restore_window_size(store: &UiRect, node: &mut Node) {
    node.left = store.left;
    node.top = store.top;
    node.width = store.right;
    node.height = store.bottom;
}


/// separate system, so that windows can be minimized by other means as well
/// TODO: implement hide as well?
pub fn apply_window_status_change(
    mut change_message: MessageReader<UiWindowsStatusChangeRequest>,
    mut q: Query<(&mut UiWindowState, &mut Node), With<UiWindow>>,
) {
    for message in change_message.read() {
        if let Ok((mut state, mut node)) = q.get_mut(message.window) {
            match (state.status, message.status) {
                (UiWindowStatus::Normal, UiWindowStatus::Minimized) => {
                    state.normal_size = save_window_size(&node);
                    node.height = Val::Px(HEIGHT_TITLE_BAR[state.ui_size]);
                    state.status = UiWindowStatus::Minimized;
                    state.set_focus(false);
                },
                (UiWindowStatus::Normal, UiWindowStatus::Maximized) => {
                    state.normal_size = save_window_size(&node);
                    node.left = Val::Px(0.);
                    node.top = Val::Px(0.);
                    node.width = Val::Percent(100.);
                    node.height = Val::Percent(100.);
                    state.status = UiWindowStatus::Maximized;
                    state.set_focus(true);
                },
                (UiWindowStatus::Minimized, UiWindowStatus::Normal) | 
                (UiWindowStatus::Minimized, UiWindowStatus::Minimized)=> {
                    restore_window_size(&state.normal_size, &mut node);
                    state.status = UiWindowStatus::Normal;
                    state.set_focus(true);
                },
                (UiWindowStatus::Minimized, UiWindowStatus::Maximized) => {
                    node.left = Val::Px(0.);
                    node.top = Val::Px(0.);
                    node.width = Val::Percent(100.);
                    node.height = Val::Percent(100.);
                    state.status = UiWindowStatus::Maximized;
                    state.set_focus(true);
                }
                (UiWindowStatus::Maximized, UiWindowStatus::Normal) |
                (UiWindowStatus::Maximized, UiWindowStatus::Maximized) => {
                    restore_window_size(&state.normal_size, &mut node);
                    state.status = UiWindowStatus::Normal;
                    state.set_focus(true);
                },
                (UiWindowStatus::Maximized, UiWindowStatus::Minimized) => {
                    node.height = Val::Px(HEIGHT_TITLE_BAR[state.ui_size]);
                    state.status = UiWindowStatus::Minimized;
                    state.set_focus(false);
                }
                _ => {}
            }
        }
    }
}

pub fn maximize_windows(
    mut q: Query<(&Interaction, &ChildOf), (Changed<Interaction>, With<UiWindowMaximizeButton>)>,
    mut windows: Query<Entity, With<UiWindow>>,
    mut change_message: MessageWriter<UiWindowsStatusChangeRequest>,
) {
    for (interaction, parent_titelbar) in &mut q {
        if *interaction == Interaction::Pressed {
            if let Ok(window) = windows.get_mut(parent_titelbar.get()) {
                change_message.write(UiWindowsStatusChangeRequest {
                    window: window,
                    status: UiWindowStatus::Maximized,
                });
            }
        }
    }
    
}