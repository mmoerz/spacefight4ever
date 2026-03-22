use bevy::prelude::*;
use crate::ui::window::systems::minmax::*;
use crate::ui::window::systems::drag_system::*;
use crate::ui::window::systems::resize::*;

use crate::ui::window::window::*;

pub struct UiWindowPlugin;

impl Plugin for UiWindowPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<UiWindowsStatusChangeRequest>()
            .add_systems(Update,
            //start_drag,
            //drag_windows,
            //stop_drag,
            // start_resize,
            // //resize_windows,
            minimize_windows)
            .add_systems(Update, apply_window_status_change)
            .add_systems(Update, maximize_windows);
            //apply_window_status_change));
            //maximize_windows));

    }
}