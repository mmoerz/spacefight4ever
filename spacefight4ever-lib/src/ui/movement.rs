use avian3d::parry::query::point;
use bevy::{
    input::{
        gestures::*,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    prelude::*,
};

use crate::ui::camera::{OrbitCamera, OrbitCameraTarget};

// Interaction I want
// First click → pick XZ position on ground plane
// Mouse move → adjust Y height
// Preview
// rectangle on base plane
// vertical line up to current height
// Second click → finalize 3D point
// This is a clean state machine problem.

#[derive(Resource, Default)]
pub struct MovementTarget {
    pub base: Option<Vec3>,
    pub height: f32,
    pub active: bool,
    pub spawned: bool,
    
    pub drag_start_mouse_y: f32,  // NEW
    pub drag_start_height: f32,   // NEW
}

pub enum MovementPlacementState {
    Idle,
    PickingBase {base : Vec3},
    AdjustingHeight {height : f32},
}

/// 3d point in space to move to
#[derive(Component)]
pub struct MovementTargetMarker;

/// line from (ship) orbitcameratarget to movement target
#[derive(Component)]
pub struct MovementTargetPathLine;

/// target to move to
#[derive(Resource, Default)]
pub struct MovementCommand {
    pub target: Option<Vec3>,
}

/// point on ground plane
#[derive(Component)]
pub struct MovementTargetPreviewBase;

/// point above ground plane (target)
#[derive(Component)]
pub struct MovementTargetPreview;

/// line from base to target (height preview)
#[derive(Component)]
pub struct MovementTargetPreviewLine;

/// line from base to target (path preview)
#[derive(Component)]
pub struct MovementTargetPreviewPath;

fn movement_height_input_system(
    mut state: ResMut<MovementTarget>,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    ground: Single<&GlobalTransform, With<GroundPlane>>,
    mut movement_command: ResMut<MovementCommand>,
) {
    let (camera, camera_transform) = *camera_query;

    let Some(cursor) = window.cursor_position() else { return; };
    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor) else { return; };

    // --- click base ---
    if mouse.just_pressed(MouseButton::Left) && state.base.is_none() {
        if let Some(point) = ray.plane_intersection_point(
            ground.translation(),
            InfinitePlane3d::new(ground.up()),
        ) {
            state.base = Some(point);
            state.height = point.y;
            state.active = true;

            // start drag reference
            state.drag_start_mouse_y = cursor.y;
            state.drag_start_height = state.height;
        }
        return;
    }

    // --- drag height (STABLE) ---
    if let Some(base) = state.base {
        //if mouse.pressed(MouseButton::Left) {
            let cam_pos = camera_transform.translation();
            let distance = cam_pos.distance(base);

            // tune these
            let base_sensitivity = 0.002;
            let sensitivity = base_sensitivity * distance;

            let delta_y = cursor.y - state.drag_start_mouse_y;

            state.height = state.drag_start_height - delta_y * sensitivity;
        //}

        // confirm
        if mouse.just_pressed(MouseButton::Left) {
            state.active = false;

            let final_pos = Vec3::new(base.x, state.height, base.z);
            println!("Final 3D position: {:?}", final_pos);

            movement_command.target = Some(final_pos);

            state.base = None;
        }
    }
}

/// scale the pseudo-gizmos for the preview
fn preview_scale(cam_pos: Vec3, point: Vec3) -> f32 {
    let dist = cam_pos.distance(point);

    // tune this constant
    (dist * 0.05).clamp(0.05, 2.0)
}

fn spawn_movement_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // target cube
    commands.spawn((
        MovementTargetMarker,
        Mesh3d(meshes.add(Cuboid::new(0.25, 0.25, 0.25))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.2, 0.2),
            unlit: true,
            ..default()
        })),
        Visibility::Visible,
    ));

    // path line
    commands.spawn((
        MovementTargetPathLine,
        Mesh3d(meshes.add(Cuboid::new(1.0, 0.01, 0.05))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.8, 1.0),
            unlit: true,
            ..default()
        })),
        Visibility::Visible,
    ));
}

fn sync_movement_visuals(
    movement: Res<MovementCommand>,
    cam: Single<&GlobalTransform, With<OrbitCamera>>,
    camera_target: Query<&GlobalTransform, With<OrbitCameraTarget>>,
    mut marker_q: Query<&mut Transform, (With<MovementTargetMarker>, Without<MovementTargetPathLine>)>,
    mut line_q: Query<&mut Transform, (With<MovementTargetPathLine>, Without<MovementTargetMarker>)>,
) {
    let Some(target) = movement.target else { return; };

    let Ok(cam_target) = camera_target.single() else { return; };

    let Ok(mut marker) = marker_q.single_mut() else { return; };
    let Ok(mut line) = line_q.single_mut() else { return; };

    let start = cam_target.translation();
    let dir = target - start;
    let length = dir.length();

    // --- marker ---
    marker.translation = target;

    let cam_pos = cam.translation();
    let thickness: f32 = preview_scale(cam_pos, target);

    // --- line ---
    if length > 0.001 {
        let mid = start + dir * 0.5;

        line.translation = mid;
        line.rotation = Quat::from_rotation_arc(Vec3::Z, dir.normalize());
        line.scale = Vec3::new(thickness, thickness, length);
    }
}

fn sync_preview_base(
    state: Res<MovementTarget>,
    cam: Single<&GlobalTransform, With<OrbitCamera>>,
    mut q: Query<&mut Transform, With<MovementTargetPreviewBase>>,
) {
    let Some(base) = state.base else { return; };

    let Ok(mut t) = q.single_mut() else { return; };

    let cam_pos = cam.translation();
    let thickness = preview_scale(cam_pos, base) * 0.2;

    t.translation = base + Vec3::Y * 0.01;
    t.scale = Vec3::splat(thickness);
}

fn sync_preview_top(
    state: Res<MovementTarget>,
    cam: Single<&GlobalTransform, With<OrbitCamera>>,
    mut q: Query<&mut Transform, With<MovementTargetPreview>>,
) {
    let Some(base) = state.base else { return; };

    let top = Vec3::new(base.x, state.height, base.z);

    let Ok(mut t) = q.single_mut() else { return; };

    let cam_pos = cam.translation();
    let scale = preview_scale(cam_pos, base);

    t.translation = top + Vec3::Y * 0.01;
    t.scale = Vec3::splat(scale);
}

fn sync_preview_line(
    state: Res<MovementTarget>,
    cam: Single<&GlobalTransform, With<OrbitCamera>>,
    mut q: Query<&mut Transform, With<MovementTargetPreviewLine>>,
) {
    let Some(base) = state.base else { return; };

    let top = Vec3::new(base.x, state.height, base.z);

    let Ok(mut t) = q.single_mut() else { return; };

    let cam_pos = cam.translation();
    let thickness: f32 = preview_scale(cam_pos, base) * 0.2;

    let dir = top - base;
    let length = dir.length();

    if length > 0.001 {
        let mid = base + dir * 0.5;

        t.translation = mid + Vec3::Y * 0.01;
        t.rotation = Quat::from_rotation_arc(Vec3::Z, dir.normalize());
        t.scale = Vec3::new( thickness, thickness, length);
    }
}

fn sync_preview_path(
    state: Res<MovementTarget>,
    cam: Single<&GlobalTransform, With<OrbitCamera>>,
    camera_target: Query<&GlobalTransform, With<OrbitCameraTarget>>,
    mut q: Query<&mut Transform, With<MovementTargetPreviewPath>>,
) {
    let Some(base) = state.base else { return; };

    let Ok(cam_target) = camera_target.single() else { return; };
    let Ok(mut t) = q.single_mut() else { return; };

    let cam_pos = cam.translation();
    let thickness: f32 = preview_scale(cam_pos, base) * 0.2;    

    let top = Vec3::new(base.x, state.height, base.z);
    let start = cam_target.translation();

    let dir = top - start;
    let length = dir.length();

    if length > 0.001 {
        let mid = start + dir * 0.5;

        t.translation = mid + Vec3::Y * 0.01;
        t.rotation = Quat::from_rotation_arc(Vec3::Z, dir.normalize());
        t.scale = Vec3::new( thickness, thickness, length);
    }
}

/// spawn the visuals once and hide/show them when needed
fn spawn_all_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let line_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.8, 0.8),
        unlit: true,
        ..default()
    });

    let point_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2),
        unlit: true,
        ..default()
    });

    // --- preview base point in ground plane ---
    commands.spawn((
        MovementTargetPreviewBase,
        Mesh3d(meshes.add(Cuboid::new(0.2, 0.2, 0.2))),
        MeshMaterial3d(point_mat.clone()),
        Visibility::Hidden,
    ));

    // --- preview top which is the target point ---
    commands.spawn((
        MovementTargetPreview,
        Mesh3d(meshes.add(Cuboid::new(0.2, 0.2, 0.2))),
        MeshMaterial3d(point_mat.clone()),
        Visibility::Hidden,
    ));

    // --- preview vertical line from base to target point ---
    commands.spawn((
        MovementTargetPreviewLine,
        Mesh3d(meshes.add(Cuboid::new(0.05, 0.05, 1.0))),
        MeshMaterial3d(line_mat.clone()),
        Visibility::Hidden,
    ));

    // --- preview path ---
    commands.spawn((
        MovementTargetPreviewPath,
        Mesh3d(meshes.add(Cuboid::new(0.05, 0.05, 1.0))),
        MeshMaterial3d(line_mat.clone()),
        Visibility::Hidden,
    ));

    // --- final marker ---
    commands.spawn((
        MovementTargetMarker,
        Mesh3d(meshes.add(Cuboid::new(0.2, 0.2, 0.2))),
        MeshMaterial3d(point_mat.clone()),
        Visibility::Hidden,
    ));

    // --- final path ---
    commands.spawn((
        MovementTargetPathLine,
        Mesh3d(meshes.add(Cuboid::new(0.05, 0.05, 1.00))),
        MeshMaterial3d(line_mat),
        Visibility::Hidden,
    ));
}

/// system to update the preview visibility based on the state of the preview
fn update_preview_visibility(
    state: Res<MovementTarget>,
    mut q: Query<&mut Visibility, Or<(
        With<MovementTargetPreviewBase>,
        With<MovementTargetPreview>,
        With<MovementTargetPreviewLine>,
        With<MovementTargetPreviewPath>,
    )>>,
) {
    let visible = state.active;

    for mut vis in &mut q {
        *vis = if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// system to update the movement visibility
fn update_movement_visibility(
    movement: Res<MovementCommand>,
    mut q: Query<&mut Visibility, Or<(
        With<MovementTargetMarker>,
        With<MovementTargetPathLine>,
    )>>,
) {
    let visible = movement.target.is_some();

    for mut vis in &mut q {
        *vis = if visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

#[derive(Component)]
struct GroundPlane;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        GroundPlane,
    ));
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<MovementTarget>()
            .init_resource::<MovementCommand>()
            .add_systems(Startup, (
                setup,
                spawn_all_visuals,
            ))
            .add_systems(Update, (
                movement_height_input_system,
                update_preview_visibility,
                update_movement_visibility,
                (
                    sync_preview_base,
                    sync_preview_line,
                    sync_preview_top,
                    sync_preview_path,
                ).chain(),
                sync_movement_visuals,
                ))
            ;
    }
}