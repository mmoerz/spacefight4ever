use bevy::{ecs::relationship::Relationship, prelude::*};

use crate::component::*;
use crate::structs::*;
use crate::consts::HEIGHT_TITLE_BAR;
use crate::window::window::get_window_node;

#[derive(Message)]
pub struct UiWindowsStatusChangeRequest {
    window: Entity,
    status: UiWindowStatus,
}


// TODO: implement making the titlebar smaller and moving it to the bottom corner, but
// this will need some sort of tracking what windows have already been minimized
pub fn minimize_windows(
    q: Query<(&Interaction, &ChildOf), (Changed<Interaction>, With<UiWindowMinimizeButton>)>,
    parent_query: Query<&ChildOf>,
    windows: Query<Entity, With<UiWindow>>,
    mut change_message: MessageWriter<UiWindowsStatusChangeRequest>,
) {
    for (interaction, button_container) in q {
        if *interaction == Interaction::Pressed {
            if let Some(window_entity) = get_window_node(&windows, button_container.get(), &parent_query) {
                change_message.write(UiWindowsStatusChangeRequest {
                    window: window_entity,
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
    parent_query: Query<&ChildOf>,
    mut windows: Query<Entity, With<UiWindow>>,
    mut change_message: MessageWriter<UiWindowsStatusChangeRequest>,
) {
    for (interaction, button_container) in &mut q {
        if *interaction == Interaction::Pressed {
             if let Ok(titlebar) = parent_query.get(button_container.get()) {
                println!("parent_tbar: {:?}", titlebar);
                if let Ok(parent_window) = parent_query.get(titlebar.get()) {
                    if let Ok(window) = windows.get_mut(parent_window.get()) {
                        change_message.write(UiWindowsStatusChangeRequest {
                            window: window,
                            status: UiWindowStatus::Maximized,
                        });
                    }
                }
            }
        }
    }
}