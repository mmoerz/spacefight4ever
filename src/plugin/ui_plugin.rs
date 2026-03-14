use bevy::prelude::*;
use crate::ui;
use crate::ui::messages::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ui::state::UiState>()
            .init_resource::<ui::dialog_stack::DialogStack>()

            .add_message::<DialogRequest>()
            .add_message::<DialogResult>()

            .add_systems(Startup, ui::layers::spawn_ui_camera)
            .add_systems(Startup, ui::layers::spawn_ui_roots)
            .add_systems(Update, (
                ui::dialog_manager::dialog_request_system,
                ui::dialog_manager::dialog_spawn_system,
                ui::systems::dialog::dialog_button_system
            ))
            .add_systems(Update, ui::systems::button::button_system)
            //.add_systems(Update, ui::animation::animate_ui)
            .add_systems(Update, ui::state::input_routing_system)
            .add_systems(Update, ui::dialog_manager::exit_on_confirm);
    }
}