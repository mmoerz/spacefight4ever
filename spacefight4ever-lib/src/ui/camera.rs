
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

/// contains the information about the 3d camera focal point
/// for 3rd person view
/// TODO: research if the target is really needed, should be possible to work with Entity query to get the target
#[derive(Component)]
pub struct OrbitCamera {
    target: Entity,       // the target we orbit around
    distance: f32,        // distance from target
    yaw: f32,             // horizontal angle
    pitch: f32,           // vertical angle
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
        let speed = sensitivity * orbit.distance.sqrt() * 1.6;

        orbit.yaw -= delta.x * speed;
        orbit.pitch -= delta.y * speed;

        orbit.pitch = orbit.pitch.clamp(
            -89.9_f32.to_radians(),
            89.9_f32.to_radians(),
        );
    }
}

// zoom in and out system
// TODO: maybe add a modifier key to multiply camera zoom
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

        let target = target_transform.translation;

        let rotation =
            Quat::from_axis_angle(Vec3::Y, orbit.yaw) *
            Quat::from_axis_angle(Vec3::X, orbit.pitch);

        transform.translation =
            target + rotation * Vec3::new(0.0, 0.0, orbit.distance);

        transform.look_at(target, Vec3::Y);
    }
}

/// set up the orbit camera 
fn assign_camera_target_system(
    mut camera_query: Query<&mut OrbitCamera>,
    target_query: Query<Entity, With<OrbitCameraTarget>>,
) {
    let Ok(target) = target_query.single() else { return; };

    for mut orbit in &mut camera_query {
        if orbit.get_target() == Entity::PLACEHOLDER {
            orbit.set_target(target);
            println!("Camera target assigned: {:?}", target);
        }
    }
}

pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                assign_camera_target_system,
                orbit_camera_input_system,
                orbit_camera_zoom_system,
                orbit_camera_transform_system
                ).chain()
            );
    }
}