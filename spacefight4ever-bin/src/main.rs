

// #[cfg(target_os = "linux")]
// unsafe fn configure_linker() {
//     std::env::set_var("RUSTFLAGS", "-C link-arg=-fuse-ld=lld -C target-cpu=native");
//     std::env::set_var("CC", "clang");
// }


// #[cfg(not(target_os = "linux"))]
// fn configure_linker() {
//     // No-op for non-Linux targets
// }


use bevy::{
    prelude::*,
    input_focus::{tab_navigation::TabNavigationPlugin, InputDispatchPlugin}
};
use avian3d::prelude::*;
use bevy_ui_widgets::UiWidgetsPlugins;


use spacefight4ever_ui::{
    ui::assets::assets::setup_ui_theme,
    plugins::{UiAssetsPlugin, UiAtlasButtonPlugin, UiAtlasWindowPlugin}
};

use spacefight4ever_lib::prelude::*;
use spacefight4ever_lib::config::environment::*;
use spacefight4ever_lib::game::player::gameassets::GameAssetsPlugin;
use spacefight4ever_lib::game::player::player::PlayerPlugin;
use spacefight4ever_lib::ui::camera::{OrbitCamera, OrbitCameraTarget, GameCameraPlugin};
use spacefight4ever_lib::config::environment::ConfigPlugin;
use spacefight4ever_lib::ui::overlay::slider::{UiSliderPlugin};
use spacefight4ever_lib::ui::overlay::settings::UiSettingsPlugin;
use spacefight4ever_lib::ui::movement::MovementPlugin;


// to consider:
/// TODO: get viewport to world coordinates for travelling - example: 3d_viewport_to_world
/// TODO: shake viewport via switching sub view offset (example: camera_sub_view)

fn main() {
    App::new()

        .add_plugins((
            DefaultPlugins.set(
            bevy::asset::AssetPlugin {
            file_path: AppConfig::default().asset_path.into(),
            ..default()
            }),
            PhysicsPlugins::default(),
            UiWidgetsPlugins,
            InputDispatchPlugin,
            TabNavigationPlugin
        ))
        // config and settings plugins
        .add_plugins(ConfigPlugin)
        .add_plugins(GameAssetsPlugin) // should load the necessary assets - currently only for the player ship

        // camera setup
        .add_plugins(GameCameraPlugin)

        // -ui crate plugins
        .add_plugins(UiAssetsPlugin)
        .add_plugins(UiAtlasButtonPlugin)
        .add_plugins(UiAtlasWindowPlugin)

        // -lib plugins
        .add_plugins(UiPlugin)

        .add_plugins(UiSliderPlugin)
        .add_plugins(UiSettingsPlugin)

        .add_plugins(PlayerPlugin)
        .add_plugins(MovementPlugin)

        .add_systems(Startup, setup_ui_theme)
        //.add_systems(Startup, setup)
        .add_systems(Startup, testsetup)

        .run();
}

/// set up a simple 3D scene
fn testsetup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ship
    // let cube = 
    commands.spawn((
        // Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        // MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        SceneRoot(asset_server.load("ships/models/Spitfire.glb#Scene0")),
        Transform::from_xyz(0.0, 0.5, 0.0),
        // OrbitCameraTarget,
    )).id();

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}