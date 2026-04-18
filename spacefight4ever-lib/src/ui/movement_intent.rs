use avian3d::prelude::*;
use bevy::{
    input::{
        gestures::*,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    }, math::VectorSpace, prelude::*
};

use crate::game::player::playership::*;
use crate::ui::camera::{OrbitCamera, OrbitCameraTarget};

// Interaction I want
// First click → pick XZ position on ground plane
// Mouse move → adjust Y height
// Preview
// rectangle on base plane
// vertical line up to current height
// Second click → finalize 3D point
// This is a clean state machine problem.

// possible improvements:
// dashed lines (shader or segmented mesh)
// animation along path (maybe not)
// multiple waypoints

#[derive(Resource, Default)]
pub struct MovementPlacementData {
    pub base : Vec3,
    pub height : f32,
   
    pub drag_start_mouse_y: f32,  // for storing the initial y value
    pub drag_start_height: f32,   // for storing the resulting height
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MovementPlacementState {
    #[default]
    Idle,
    PlacingHeight,
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
    pub target: Vec3,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MovementCommandState {
    #[default]
    Idle,
    Moving,
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

fn movement_base_input_system(
    mut next_state: ResMut<NextState<MovementPlacementState>>,
    mut data: ResMut<MovementPlacementData>,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&Camera, &GlobalTransform), With<Camera3d>>,
    ground: Single<&GlobalTransform, With<GroundPlane>>,
) {
    let (camera, camera_transform) = *camera_query;
    let Some(cursor) = window.cursor_position() else { return; };

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor) else { return; };

    // --- click base ---
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(point) = ray.plane_intersection_point(
            ground.translation(),
            InfinitePlane3d::new(ground.up()),
        ) {
            data.base = point;
            data.height = point.y;

            // start drag reference
            data.drag_start_mouse_y = cursor.y;
            data.drag_start_height = data.height;

            next_state.set(MovementPlacementState::PlacingHeight);
        }
    }
}

fn movement_height_input_system(
    mut next_state: ResMut<NextState<MovementPlacementState>>,
    mut move_state: ResMut<NextState<MovementCommandState>>,
    mut data: ResMut<MovementPlacementData>,
    mouse: Res<ButtonInput<MouseButton>>,
    window: Single<&Window>,
    camera_query: Single<(&GlobalTransform), With<Camera3d>>,
    mut movement_command: ResMut<MovementCommand>,
) {
    let camera_transform = *camera_query;

    let Some(cursor) = window.cursor_position() else { return; };

    // --- drag height (STABLE) ---
    //if mouse.pressed(MouseButton::Left) {
    let cam_pos = camera_transform.translation();
    let distance = cam_pos.distance(data.base);

    // tune these
    let base_sensitivity = 0.002;
    let sensitivity = base_sensitivity * distance;

    let delta_y = cursor.y - data.drag_start_mouse_y;

    data.height = data.drag_start_height - delta_y * sensitivity;
    //}

    // confirm
    if mouse.just_pressed(MouseButton::Left) {
        let final_pos = Vec3::new(data.base.x, data.height, data.base.z);
        println!("Final 3D position: {:?}", final_pos);

        movement_command.target = final_pos;
        move_state.set(MovementCommandState::Moving);

        data.base = Vec3::ZERO;
        data.height = 0.0;

        next_state.set(MovementPlacementState::Idle);
    }
}

/// scale the pseudo-gizmos for the preview
fn preview_scale(cam_pos: Vec3, point: Vec3) -> f32 {
    let dist = cam_pos.distance(point);

    // tune this constant
    (dist * 0.05).clamp(0.05, 2.0)
}

fn sync_movement_visuals(
    movement: Res<MovementCommand>,
    cam: Single<&GlobalTransform, With<OrbitCamera>>,
    camera_target: Query<&GlobalTransform, With<OrbitCameraTarget>>,
    mut marker_q: Query<&mut Transform, (With<MovementTargetMarker>, Without<MovementTargetPathLine>)>,
    mut line_q: Query<&mut Transform, (With<MovementTargetPathLine>, Without<MovementTargetMarker>)>,
) {
    let target= movement.target;

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
    state: Res<MovementPlacementData>,
    cam: Single<&GlobalTransform, With<OrbitCamera>>,
    mut q: Query<&mut Transform, With<MovementTargetPreviewBase>>,
) {
    let base = state.base;

    let Ok(mut t) = q.single_mut() else { return; };

    let cam_pos = cam.translation();
    let thickness = preview_scale(cam_pos, base) * 0.2;

    t.translation = base + Vec3::Y * 0.01;
    t.scale = Vec3::splat(thickness);
}

fn sync_preview_top(
    state: Res<MovementPlacementData>,
    cam: Single<&GlobalTransform, With<OrbitCamera>>,
    mut q: Query<&mut Transform, With<MovementTargetPreview>>,
) {
    let base = state.base;

    let top = Vec3::new(base.x, state.height, base.z);

    let Ok(mut t) = q.single_mut() else { return; };

    let cam_pos = cam.translation();
    let scale = preview_scale(cam_pos, base);

    t.translation = top + Vec3::Y * 0.01;
    t.scale = Vec3::splat(scale);
}

fn sync_preview_line(
    state: Res<MovementPlacementData>,
    cam: Single<&GlobalTransform, With<OrbitCamera>>,
    mut q: Query<&mut Transform, With<MovementTargetPreviewLine>>,
) {
    let base = state.base;

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
    state: Res<MovementPlacementData>,
    cam: Single<&GlobalTransform, With<OrbitCamera>>,
    camera_target: Query<&GlobalTransform, With<OrbitCameraTarget>>,
    mut q: Query<&mut Transform, With<MovementTargetPreviewPath>>,
) {
    let base = state.base;

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
    state: Res<State<MovementPlacementState>>,
    mut q: Query<&mut Visibility, Or<(
        With<MovementTargetPreviewBase>,
        With<MovementTargetPreview>,
        With<MovementTargetPreviewLine>,
        With<MovementTargetPreviewPath>,
    )>>,
) {
    let visible = *state.get() == MovementPlacementState::PlacingHeight;

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
    state: Res<State<MovementCommandState>>,
    mut q: Query<&mut Visibility, Or<(
        With<MovementTargetMarker>,
        With<MovementTargetPathLine>,
    )>>,
) {
    for mut vis in &mut q {
        *vis = if *state.get() == MovementCommandState::Moving {
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
        Visibility::Hidden,
    ));
}

fn ship_force_steering_system(
    movement: Res<MovementCommand>,
    mut query: Query<(Entity, Forces), With<PlayerShip>>, // forces is not a component only a query_data
    transforms: Query<&GlobalTransform, With<PlayerShip>>,
) {
    let target = movement.target;

    for (entity, mut force) in &mut query {
        let Ok(transform) = transforms.get(entity) else { continue; };
        let angular = force.angular_velocity();
        let linear = force.linear_velocity();

        let position = transform.translation();
        let forward = transform.forward();

        let to_target = target - position;
        let distance = to_target.length();

        if distance < 0.5 {
            //force.apply_force(-linear * 10.0);
            continue;
        }

        let desired_dir = to_target.normalize();

        // --- ALIGNMENT ---
        let alignment = forward.dot(desired_dir);

        let rotation_axis = forward.cross(desired_dir);

        // turn ship toward target
        //force.apply_torque(rotation_axis.normalize_or_zero() * 5.0 - angular.abs() * 3.0);

        // --- THRUST ---
        let thrust_power = 40.0;
        //let thrust_factor = alignment.clamp(0.0, 1.0);
        let desired_velocity = desired_dir * thrust_power;
        let steering: Vec3 = desired_velocity - linear;

        force.apply_force(steering.clamp_length_max(thrust_power));
    }
}

fn debug_velocity(query: Query<&LinearVelocity, With<PlayerShip>>) {
    if let Ok(v) = query.single() {
        println!("vel: {:?}", v);
    }
}

fn debug_ship_transform(
    ship: Single<&GlobalTransform, With<PlayerShip>>,
    marker: Query<&GlobalTransform, With<MovementTargetMarker>>,
) {
    if let Ok(target) = marker.single() {
        println!("ship world pos: {:?}, target: {:?}", ship.translation(), target.translation());
    } else {
        println!("ship world pos: {:?}", ship.translation());
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<MovementPlacementState>()
            .init_state::<MovementCommandState>()
            .init_resource::<MovementPlacementData>()
            .init_resource::<MovementCommand>()
            .add_systems(Startup, (
                setup,
                spawn_all_visuals,
            ))
            .add_systems(Update, (
                movement_base_input_system.run_if(in_state(MovementPlacementState::Idle)),
                movement_height_input_system.run_if(in_state(MovementPlacementState::PlacingHeight))
            ))
            .add_systems(OnEnter(MovementCommandState::Moving), update_movement_visibility)
            .add_systems(OnEnter(MovementCommandState::Idle), update_movement_visibility)
            .add_systems(Update, (
                update_preview_visibility,
                (
                    sync_preview_base,
                    sync_preview_line,
                    sync_preview_top,
                    sync_preview_path,
                ).chain(),
                sync_movement_visuals,
                ship_force_steering_system,
                //debug_velocity,
                debug_ship_transform,
                ))
            ;
    }
}