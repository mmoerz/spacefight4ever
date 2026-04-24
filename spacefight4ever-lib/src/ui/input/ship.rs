use bevy::prelude::*;
use avian3d::prelude::*;

use crate::game::{player::playership::PlayerShip};
use crate::game::ship::modules::propulsion::PropulsionStat;
use crate::game::ship::definitions::{
    ship_definition::{ShipDefinition, ShipDefinitionIndex, ShipModel},
    ship_models::{ShipModelIndex},
};
use crate::game::ship::modules::stats::Stat;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SpaceshipController {
    pub thrust_multiplier: f32,
}

impl Default for SpaceshipController {
    /// returns default for a SpaceshipController
    /// move_speed 50, rotation_speed 2
    /// 
    fn default() -> Self {
        Self {
            thrust_multiplier: 1.0,
        }
    }
}

pub fn spaceship_movement_system (
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, Forces), (With<SpaceshipController>, With<PlayerShip>)>, // forces is not a component only a query_data
    //index: Res<ShipModelIndex>,
    mut propulsion_query: Query<&mut PropulsionStat, (With<SpaceshipController>, With<PlayerShip>)>,
    defs: Res<Assets<ShipDefinition>>,
    index: Res<ShipDefinitionIndex>,
    transform_query: Query<(&Transform, &ShipModel, &SpaceshipController)>,
) {
    for (entity, mut force) in &mut query {
        let Ok(mut propulsion) = propulsion_query.get_mut(entity) else {
                continue; 
            };
        let Ok((transform, ship_model, controller)) = 
            transform_query.get(entity) else { continue; };
        
        // --- Linear Movement (Thrust) ---
        let mut thrust = Vec3::ZERO;
        let mut prop_thrust = 0.0;
        if keyboard.pressed(KeyCode::ArrowUp) { 
            thrust -= transform.forward().as_vec3();
            prop_thrust = propulsion.max();
        }
        if keyboard.pressed(KeyCode::ArrowDown) { 
            thrust += transform.forward().as_vec3();
            prop_thrust = -propulsion.max();
        }
        
        propulsion.set(prop_thrust);
        //let max_accel = propulsion.calculate_accelartion_max(ship_model, &index, &defs);

        // Apply force (resetting each frame to prevent infinite acceleration)
        // lol this can't be max cruise speed, because the force is normally measured in N
        // since there is no mass here, this is just a half truth
        force.apply_force(thrust * controller.thrust_multiplier * propulsion.get());

        // --- Angular Movement (Rotation/Pitch/Yaw) ---
        let mut rotation = Vec3::ZERO;
        if keyboard.pressed(KeyCode::KeyW) { rotation.x += 1.0; } // Pitch
        if keyboard.pressed(KeyCode::KeyS) { rotation.x -= 1.0; }
        if keyboard.pressed(KeyCode::KeyA) { rotation.y += 1.0; } // Yaw
        if keyboard.pressed(KeyCode::KeyD) { rotation.y -= 1.0; }
        if keyboard.pressed(KeyCode::KeyQ) { rotation.z += 1.0; } // Roll
        if keyboard.pressed(KeyCode::KeyE) { rotation.z -= 1.0; }

        force.apply_torque(transform.rotation * (rotation * 1.0));
    }
}

// //
// let speed = force.linear_velocity().length();
// let x = speed / profile.cruise_speed;
// let value = (x / (1.0 + x)).clamp(0.0, 1.0);

// essential setup
// commands.spawn((
//     SceneRoot(asset_server.load("ships/models/Spitfire.glb#Scene0")),
//     SpaceshipController::default(),
//     // Physics Components
//     RigidBody::Dynamic,
//     Collider::cuboid(1.0, 1.0, 2.0), // Adjust to your GLB size
//     ExternalForce::default(),
//     ExternalTorque::default(),
//     // Damping mimics "flight stabilization" computers
//     LinearDamping(0.5), 
//     AngularDamping(1.0), 
// ));

// 1. The "Terminal Velocity" Balance
// In space (vacuum), an object would accelerate forever. However, since you are using LinearDamping and AngularDamping, you create a "drag" effect that eventually cancels out your thrust.

//     Top Speed Calculation: Your ship reaches its limit when:
//     Force (move_speed) == Damping * Current Velocity.
//     The Feel:
//         Increase move_speed → You reach top speed faster.
//         Decrease Damping → Your top speed becomes higher, but the ship feels "drifter" and harder to stop.

// 2. Mass and Inertia
// Avian3D automatically calculates Mass and Inertia based on your Collider.

//     If your Spitfire.glb has a huge collider, the same move_speed (Force) will move it much slower than a small ship.
//     Best Practice: Add the Mass component explicitly if you want to override the automatic calculation and keep handling consistent across different model sizes.

// 3. How to add a "Hard" Speed Cap
// If you want a strict maximum speed (e.g., "The ship cannot exceed 100 m/s"), you shouldn't rely on damping alone. You should add a system to clamp the LinearVelocity:
// rust

// fn clamp_velocity(mut query: Query<&mut LinearVelocity>) {
//     let max_speed = 100.0;
//     for mut vel in query.iter_mut() {
//         let speed = vel.length();
//         if speed > max_speed {
//             vel.0 *= max_speed / speed;
//         }
//     }
// }

// Use code with caution.
// Summary of what limits what:

//     move_speed: How much Thrust (Acceleration) the engines have.
//     LinearDamping: How quickly the ship naturally settles to a stop and where the "natural" speed limit is.
//     Mass: How much the ship resists changes in motion (Heavy ship = slow to start/stop).
//     MaxDistance (Avian Config): Physics engines have a default speed limit (usually very high) to prevent objects from tunneling through walls; you rarely hit this in gameplay.

// Would you like to adjust the Damping dynamically to simulate "Flight Assist" being turned on or off?

// rust

// // Example: Forward is 100% power, Strafe is 30% power
// let forward_thrust = transform.forward().as_vec3() * forward_input;
// let side_thrust = transform.right().as_vec3() * strafe_input * 0.3;

// let total_force = (forward_thrust + side_thrust).clamp_length_max(controller.move_speed);
// force.set_force(total_force);

// To make your ship feel like a "fighter," you might want a thrust-to-weight ratio (TWR) of at least 2:1.

//     Check Mass: Find your ship's mass using query.get(entity).unwrap().0.mass().
//     Calculate N: If your ship weighs 1,000 kg, you need 9,810 N of force just to hover on Earth. For a snappy space fighter, a move_speed of 20,000 N to 50,000 N would feel powerful without being uncontrollable.
