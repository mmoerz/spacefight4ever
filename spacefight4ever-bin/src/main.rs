

// #[cfg(target_os = "linux")]
// unsafe fn configure_linker() {
//     std::env::set_var("RUSTFLAGS", "-C link-arg=-fuse-ld=lld -C target-cpu=native");
//     std::env::set_var("CC", "clang");
// }


// #[cfg(not(target_os = "linux"))]
// fn configure_linker() {
//     // No-op for non-Linux targets
// }


use bevy::prelude::*;
use avian3d::prelude::*;


use spacefight4ever_lib::prelude::*;

use spacefight4ever_lib::setup;
use spacefight4ever_lib::config::environment::*;
use spacefight4ever_ui::{UiWindowPlugin, UiWindowExtensionPlugin};

fn main() {
    App::new()

        .add_plugins((
            DefaultPlugins.set(
            bevy::asset::AssetPlugin {
            file_path: get_s4fe_config().asset_path.into(),
            ..default()
            }),
            PhysicsPlugins::default()) 
        )
        .add_plugins(UiPlugin)
        .add_plugins(UiWindowPlugin)
        .add_plugins(UiWindowExtensionPlugin)
        //.add_plugins(UiWindowNinePlugin)
        .add_plugins(GamePlugin)

        .add_systems(Startup, setup)

        .run();
}