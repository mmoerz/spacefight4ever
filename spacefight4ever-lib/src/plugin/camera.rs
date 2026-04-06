use bevy::prelude::*;
use crate::ui::camera::{orbit_camera_input_system, orbit_camera_transform_system, orbit_camera_zoom_system};

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                orbit_camera_input_system,
                orbit_camera_zoom_system,
                orbit_camera_transform_system
                    .after(orbit_camera_input_system)
                    .after(orbit_camera_zoom_system),
                //orbit_camera_transform_system,
            ));
    }
}