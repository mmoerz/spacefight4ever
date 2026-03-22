use bevy::prelude::*;
use crate::ui::window::systems::minmax::*;
use crate::ui::window::systems::drag_system::*;
use crate::ui::window::systems::resize::*;

pub struct UiWindowPlugin;

impl Plugin for UiWindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            //start_drag,
            //drag_windows,
            //stop_drag,
            start_resize,
            //resize_windows,
            minimize_windows,
            apply_minimize,
            maximize_windows,
            apply_maximize,
        ));
    }
}