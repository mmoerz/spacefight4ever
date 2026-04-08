
use bevy::{
    anti_alias::fxaa::Sensitivity, input::{
        gestures::*,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    }, prelude::*
};

use crate::config::environment::AppConfig;

/// spawn the cameras
pub fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera3d"),
        Camera3d::default(),
        Camera::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        OrbitCamera {
            target: Entity::PLACEHOLDER,
            distance: 10.0,
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 0.001,
        },
    ));

    // UiCamera
    commands.spawn((
        Camera2d::default(),
        Camera { order: 1, ..default() },
        Name::new("Camera2d"),
    ));
}

#[derive(Component)]
pub struct OrbitCameraTarget;

#[derive(Component)]
pub struct OrbitCamera {
    target: Entity,       // the target we orbit around
    distance: f32,        // distance from target
    yaw: f32,             // horizontal angle
    pitch: f32,           // vertical angle
    sensitivity: f32,     // mouse sensitivity
}

impl OrbitCamera {
    pub fn set_target(&mut self, target: Entity) {
        self.target = target;
    }
    pub fn get_target(&self) -> Entity {
        self.target
    }
}

/// mouse camera movement
/// TODO: I need a way to configure the sensitivity of the camera movement
pub fn orbit_camera_input_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut motion_evr: MessageReader<MouseMotion>,
    mut query: Query<&mut OrbitCamera>,
    config: Res<AppConfig>,
) {
    if !mouse_input.pressed(MouseButton::Middle) {
        return;
    }

    let mut delta = Vec2::ZERO;
    for ev in motion_evr.read() {
        delta += ev.delta;
    }

    if delta == Vec2::ZERO {
        return;
    }

    let sensitivity = config.mouse.sensitivity;

    for mut orbit in &mut query {
        // logarithmic scaling based on distance
        //let scale = orbit.distance.max(0.001).ln_1p();
        let speed =
    config.mouse.sensitivity * orbit.distance.sqrt() * 1.6;

        orbit.yaw -= delta.x * speed;
        orbit.pitch -= delta.y * speed;

        orbit.pitch = orbit.pitch.clamp(
            -89.9_f32.to_radians(),
            89.9_f32.to_radians(),
        );
    }
}

// zoom in and out system
// TODO: logarithmic zoom probably feels more natural
pub fn orbit_camera_zoom_system(
    mut scroll_evr: MessageReader<MouseWheel>,
    mut query: Query<&mut OrbitCamera>,
    config: Res<AppConfig>,
) {
    let mut scroll = 0.0;

    for ev in scroll_evr.read() {
        scroll += ev.y;
    }

    if scroll == 0.0 {
        return;
    }

    let base = config.mouse.sensitivity;

    for mut orbit in &mut query {
                // exponential zoom (logarithmic feel)
        let speed = base * (orbit.distance + 1.0) * 4.0;

        let zoom_factor = (-scroll * speed).exp();

        orbit.distance *= zoom_factor;

        // clamp zoom range
        orbit.distance = orbit.distance.clamp(2.0, 100.0);
    }
}

pub fn orbit_camera_transform_system(
    mut query: Query<(&mut Transform, &OrbitCamera)>,
    target_query: Query<&Transform, (With<OrbitCameraTarget>, Without<OrbitCamera>)>,
) {
    //println!("camera count: {}", query.iter().count());
    for (mut transform, orbit) in &mut query {
        let Ok(target_transform) = target_query.get(orbit.target) else {
            continue;
        };

        let x = orbit.distance * orbit.pitch.cos() * orbit.yaw.sin();
        let y = orbit.distance * orbit.pitch.sin();
        let z = orbit.distance * orbit.pitch.cos() * orbit.yaw.cos();

        let target = target_transform.translation;
        transform.translation = target + Vec3::new(x, y, z);
        transform.look_at(target, Vec3::Y);
    }
}

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