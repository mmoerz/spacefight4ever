use bevy::prelude::*;

use super::dialog_events::*;
use super::dialog_stack::*;
use super::dialog_manager::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DialogStack>()
            .add_event::<DialogRequest>()
            .add_event::<DialogResult>()
            .add_systems(
                Update,
                (
                    dialog_request_system,
                    dialog_spawn_system,
                    dialog_button_system,
                ),
            );
    }
}