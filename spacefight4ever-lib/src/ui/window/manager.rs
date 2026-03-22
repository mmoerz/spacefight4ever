use bevy::prelude::*;

use super::component::*;

#[derive(Resource, Default)]
pub struct WindowManager {
    pub windows: Vec<Entity>,
    pub focused: Option<Entity>,
}

/// registers new windows with the window manager
/// 
fn register_windows(
    mut manager: ResMut<WindowManager>,
    query: Query<Entity, Added<MyWindow>>,
) {
    for entity in &query {
        manager.windows.push(entity);
    }
}

/// click to focus window
/// 
fn focus_system(
    mut manager: ResMut<WindowManager>,
    mut windows: Query<&mut MyWindow>,
) {
    if let Some(focused) = manager.focused {
        for mut win in &mut windows {
            win.focused = false;
        }

        if let Ok(mut win) = windows.get_mut(focused) {
            win.focused = true;
        }
    }
}

/// bring focused window to the front
/// 
fn z_order_system(
    mut manager: ResMut<WindowManager>,
) {
    if let Some(focused) = manager.focused {
        manager.windows.retain(|&e| e != focused);
        manager.windows.push(focused);
    }
}