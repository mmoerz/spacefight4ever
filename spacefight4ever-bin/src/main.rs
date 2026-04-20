use bevy::{
    prelude::*,
    input_focus::{tab_navigation::TabNavigationPlugin, InputDispatchPlugin}
};
use bevy_skein::SkeinPlugin;
use avian3d::prelude::*;
use avian3d::collision::collider::ColliderConstructor;
use bevy_ui_widgets::UiWidgetsPlugins;

use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};


use spacefight4ever_ui::{
    plugins::{UiAssetsPlugin, UiAtlasButtonPlugin, UiAtlasWindowPlugin, UiProgressBarPlugin},
    ui::{assets::assets::setup_ui_theme}
};

use spacefight4ever_lib::{
    config::environment::*,
    //game::player::gltf_playership::GltfPlayerShipPlugin,
    game::player::{
        gameassets::GameAssetsPlugin,
        player::PlayerPlugin,
        playership::PlayerShip,
    },
    prelude::*
};
use spacefight4ever_lib::ui::camera::{OrbitCamera, OrbitCameraTarget, GameCameraPlugin};
use spacefight4ever_lib::config::environment::ConfigPlugin;
use spacefight4ever_lib::ui::overlay::slider::{UiSliderPlugin};
use spacefight4ever_lib::ui::overlay::settings::UiSettingsPlugin;
use spacefight4ever_lib::ui::movement_intent::MovementPlugin;
use spacefight4ever_lib::ui::hud::movement_display::MovementDisplayPlugin;

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
            SkeinPlugin::default(),
            UiWidgetsPlugins,
            InputDispatchPlugin,
            TabNavigationPlugin
        ))
        // physics specific registration
        .register_type::<ColliderConstructor>()
        .register_type::<RigidBody>()
        .register_type::<Friction>()
        .register_type::<Restitution>()
        .register_type::<LinearVelocity>()
        .register_type::<AngularVelocity>()
        // Debug
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())

        // config and settings plugins
        .add_plugins(ConfigPlugin)
        .add_plugins(GameAssetsPlugin) // should load the necessary assets - currently only for the player ship

        // camera setup
        .add_plugins(GameCameraPlugin)
        .add_plugins(MovementDisplayPlugin)

        // -ui crate plugins
        .add_plugins(UiAssetsPlugin)
        .add_plugins(UiAtlasButtonPlugin)
        .add_plugins(UiAtlasWindowPlugin)
        .add_plugins(UiProgressBarPlugin)

        // -lib plugins
        .add_plugins(UiPlugin)

        .add_plugins(UiSliderPlugin)
        .add_plugins(UiSettingsPlugin)

        .add_plugins(PlayerPlugin)
        //.add_plugins(MovementPlugin)
        //.add_plugins(GltfPlayerShipPlugin)

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
    // commands.spawn((
    //     Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    //     MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
    //     //SceneRoot(asset_server.load("ships/models/Spitfire.glb#Scene0")),
    //     Transform::from_xyz(0.0, 0.0, 0.0),
    //     // OrbitCameraTarget,
    // )).id();

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        //SceneRoot(asset_server.load("ships/models/Spitfire.glb#Scene0")),
        Transform::from_xyz(8.0, 0.0, 0.0),
        // OrbitCameraTarget,
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        //SceneRoot(asset_server.load("ships/models/Spitfire.glb#Scene0")),
        Transform::from_xyz(-8.0, 0.0, 0.0),
        // OrbitCameraTarget,
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}