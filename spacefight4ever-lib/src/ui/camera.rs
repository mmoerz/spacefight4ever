
use bevy::{
    input::{
        gestures::*,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    prelude::*,
};

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

    for mut orbit in &mut query {
        orbit.yaw -= delta.x * orbit.sensitivity;
        orbit.pitch -= delta.y * orbit.sensitivity;

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
) {
    let mut scroll = 0.0;

    for ev in scroll_evr.read() {
        scroll += ev.y;
    }

    if scroll == 0.0 {
        return;
    }

    for mut orbit in &mut query {
        orbit.distance -= scroll * 0.85;

        // clamp zoom range
        orbit.distance = orbit.distance.clamp(2.0, 50.0);
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
