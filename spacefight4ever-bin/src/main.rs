use bevy::prelude::*;
use spacefight4ever_lib::prelude::*;

use spacefight4ever_lib::{ setup, trigger_exit_dialog };

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin)

        .add_systems(Startup, setup)
        .add_systems(Update, trigger_exit_dialog)

        //.add_systems(Update, crate::ui::debug::debug_print_ui_tree)
        .run();
}