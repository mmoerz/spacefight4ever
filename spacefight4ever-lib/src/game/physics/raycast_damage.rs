use bevy::prelude::*;
use avian3d::prelude::*;

use crate::game::combat::basetypes::*;
use crate::game::combat::health::*;
use crate::game::combat::health_basetypes::HealthPercents;
use crate::game::ship::weapon;
use crate::game::ship::weapon::*;

/// something that can be targeted
#[derive(Component)]
pub struct TargetAble;

#[derive(Component)]
pub struct Target;

#[derive(Message, Debug, Clone, Copy)]
pub struct WeaponFireRequest {
    pub weapon_entity: Entity,
    pub target_entity: Option<Entity>, // Some if specific target, None for no target - can't fire then if it is not an aoe weapon
}

// i need a damage_system for environment damage

// weapons should only fire when there is a fire event
// i need a damage system for a specific weapon / weapongroup against a single target
// what i need to do is to check if the module has an angle limitation
// if it has I need to check wether the target is in range / optimal range
// if the target is within the angle (cone)
// then do a raycast to check wether the target can be hit (which cannot happen if it is behind something)
// 1. Range check (cheap)
// 2. Angle / cone check (cheap)
// 3. Raycast / line-of-sight (expensive)
// 4. Apply damage

// fn weapon_fire_system_old(
//     mut fire_events: MessageReader<WeaponFireRequest>,
//     weapon_query: Query<(&Transform, &Weapon)>,
//     target_query: Query<(Entity, &Transform), With<Target>>,
//     collider_query: Query<(Entity, &Transform, &Collider)>,
//     mut damage_writer: MessageWriter<HealthDamageReceived>,
// ) {
//     for fire in fire_events.read() {
//         if let Ok((weapon_transform, weapon)) = weapon_query.get(fire.weapon_entity) {
//             if weapon.ammo_count <= 0
//             {
//                 continue;
//             }

//             // Calculate ray origin and direction but in 3d 
//             let origin = weapon_transform.translation.truncate();
//             let forward = (weapon_transform.rotation * Vec3::X)
//                 .truncate()
//                 .normalize();

//             // there is only one target for the weapon
//             let targets = if let Some(target_entity) = fire.target_entity {
//                 // Single-target fire
//                 target_query.iter().filter(|(e, _)| *e == target_entity).collect::<Vec<_>>()
//             } else {
//                 // no target, no fire (currently should not happen), later this will be aoe weapons
//                 target_query.iter().collect::<Vec<_>>()
//             };

//             // there is only one target for the weapon
//             for (target_entity, target_transform) in targets {
//                 // again in 3d not 2d
//                 let target_pos = target_transform.translation.truncate();
//                 let to_target = target_pos - origin;
//                 let distance = to_target.length();

//                 // 1️⃣ Range check
//                 if distance > weapon.range[WeaponRangeType::Max] {
//                     continue;
//                 }

//                 let efficiency = (weapon.range[WeaponRangeType::Optimal] / distance).min(1.0);
//                 let dir = to_target / distance;

//                 // 2️⃣ Cone check, if target outside, no damage will be done
//                 if let Some(max_angle) = weapon.max_angle {
//                     let cos_half = (max_angle * 0.5).cos();
//                     if forward.dot(dir) < cos_half {
//                         continue;
//                     }
//                 }

//                 // 3️⃣ Raycast / line-of-sight - again in 3d not 2d
//                 if !has_line_of_sight(origin, dir, distance, target_entity, &collider_query) {
//                     continue;
//                 }

//                 // calculate damage here
//                 // Weaponbase damage + Ammo Damage

//                 // 4️⃣ Apply damage
//                 damage_writer.write(HealthDamageReceived {
//                     entity: target_entity,
//                     damage: weapon.base_damage + weapon.ammo_type.,
//                     damabe_profile: weapon.,
//                     damage_efficiency: efficiency,
//                 });
//             }
//         }
//     }
// }

fn weapon_fire_system(
    mut fire_events: MessageReader<WeaponFireRequest>,
    mut weapon_query: Query<(&Transform, &mut Weapon, &mut Ammunition)>,
    definition_query: Query<(&WeaponStats, &WeaponBehavior)>,
    target_query: Query<(Entity, &Transform), With<Target>>,
    //collider_query: &Query<(Entity, &Transform, &Collider)>,
    spatial_query: SpatialQuery,
    mut damage_writer: MessageWriter<HealthDamageReceived>,
) {
    for event in fire_events.read() {
        let Ok((weapon_transform, mut weapon, mut ammunition)) = weapon_query.get_mut(event.weapon_entity) else { continue; };
        let Ok((weapon_stats, behavior)) = definition_query.get(weapon.weapon_definition) else { continue; };

        if weapon.cooldown > 0.0 { continue; }
        if ammunition.count <= 0 { continue; }
        ammunition.count -= 1;
        weapon.cooldown = weapon_stats.fire_rate;

        if let Some(target_entity) = event.target_entity {
            let origin = weapon_transform.translation;
            let forward = weapon_transform.forward(); //rotation.mul_vec3(Vec3::X).normalize();

            let Ok((_entity, target_transform)) = target_query.get(target_entity) else { continue; };

            let to_target = target_transform.translation - origin;
            let distance = to_target.length();
            let dir  = Dir3::new((to_target / distance).normalize()).unwrap();

            // range
            if distance > weapon_stats.range[WeaponRangeType::Max] { continue; }

            // cone
            if let Some(max_angle) = weapon_stats.max_angle {
                let cos_angle = forward.dot(dir.into()).clamp(-1.0, 1.0);
                if cos_angle < (max_angle * 0.5).cos() { continue; }
            }
            match behavior {
                WeaponBehavior::Beam => {
                    if has_line_of_sight(origin, dir, distance, target_entity, &spatial_query) { 
                    let raw_damage = compose_raw_damage(target_entity, distance, weapon_stats, &ammunition);
                    damage_writer.write(raw_damage);
                }
                }
                WeaponBehavior::Missile => {
                }
                WeaponBehavior::Projectile => {
                }
            }
        }   
    }
}

pub fn compose_raw_damage(
    entity: Entity,
    distance: f32,
    weapon_stats: &WeaponStats,
    ammunition: &Ammunition
) -> HealthDamageReceived {
    let mut damage = weapon_stats.damage + ammunition.additional_damage;
    if distance <= weapon_stats.range[WeaponRangeType::Min] || distance > weapon_stats.range[WeaponRangeType::Max] {
        damage = 0.;
    } else if distance > weapon_stats.range[WeaponRangeType::Optimal] {
        // reduce damage with the distance^2 from Optimal to max (and after it's 0)
        let over_optimal = distance - weapon_stats.range[WeaponRangeType::Optimal];
        let optimal_max = weapon_stats.range[WeaponRangeType::Max] - weapon_stats.range[WeaponRangeType::Optimal];
        let reduction = 1.0 - over_optimal / optimal_max;
        damage *= reduction.powf(2.0);
    }

    HealthDamageReceived {
        entity: entity,
        damage: damage.max(0.) as i32,
        damage_profile: ammunition.damage_profile,
        damage_efficiency: ammunition.damage_efficiency,
    }
}

// fn raycast_damage_system(
//     //avian: Res<AvianWorld>,
//     query: Query<(Entity, &Transform, &Collider, &Weapon)>,
//     mut damage_writer: MessageWriter<HealthDamageReceived>,
// ) {
    

//     let ray_origin = Vec2::new(0.0, 0.0); // example start
//     let ray_dir = Vec2::new(1.0, 0.0).normalize();
//     let ray_max_len = 100.0;

//     let mut closest_hit: Option<(Entity, f32)> = None;

//     for (entity, transform, collider) in query.iter() {
//         if let Some(distance) = intersect_ray_collider(ray_origin, ray_dir, ray_max_len, transform.translation, collider) {
//             if closest_hit.is_none() || distance < closest_hit.unwrap().1 {
//                 closest_hit = Some((entity, distance));
//             }
//         }
//     }

//     if let Some((hit_entity, distance)) = closest_hit {
//         println!("Ray hit {:?} at distance {}", hit_entity, distance);
//         damage_writer.write(HealthDamageReceived {
//             entity: hit_entity,
//             damage: 25.0,
//             damage_efficiency: 1.0,
//         });
//     }
// }

// Simple circle raycast intersection (for demo)
// fn intersect_ray_collider(
//     spatial_query: SpatialQuery,
//     ray_origin: Vec3,
//     ray_dir: Dir3,
//     max_len: f32,
//     solid: bool,
//     filter: &SpatialQueryFilter
// ) -> Option<f32> {
//     if let Some(first_hit) = spatial_query.cast_ray(ray_origin, ray_dir, max_len, solid, filter){

//     }

// }



fn print_hits(spatial_query: SpatialQuery, query: Query<&Invisible>) {
    // Ray origin and direction
    let origin = Vec3::ZERO;
    let direction = Dir3::X;

    // Configuration for the ray cast
    let max_distance = 100.0;
    let solid = true;
    let filter = SpatialQueryFilter::default();

    // Cast ray and get the first hit that matches the predicate
    let hit = spatial_query.cast_ray_predicate(origin, direction, max_distance, solid, &filter, &|entity| {
        // Skip entities with the `Invisible` component.
        !query.contains(entity)
    });

    // Print first hit
    if let Some(first_hit) = hit {
        println!("First hit: {:?}", first_hit);
    }
}

/// Abstraction for raycasting to allow mocking in tests
pub trait RayCaster {
    fn cast_ray_nearest(
        &self,
        origin: Vec3,
        dir: Dir3,
        max_distance: f32,
    ) -> Option<Entity>;
}

/// necessary for unittest mock
impl<'world, 'collider> RayCaster for SpatialQuery<'world, 'collider> {
    fn cast_ray_nearest(&self, origin: Vec3, dir: Dir3, max_distance: f32) -> Option<Entity> {
        let filter = SpatialQueryFilter::default();
        self.cast_ray(origin, dir, max_distance, false, &filter)
            .map(|RayHitData { entity, .. }| entity)
    }
}

/// Checks if a target entity is visible along a ray from the origin.
/// 
/// # Arguments
/// 
/// * `origin` - The starting point of the ray in world coordinates (Vec3).
/// * `direction` - The normalized direction of the ray (Dir3).
/// * `max_distance` - Maximum distance the ray can travel.
/// * `target` - The entity that the ray is supposed to hit.
/// * `spatial_query` - Reference to the spatial query used for raycasting.
/// 
/// # Returns
/// 
/// * `true` if the ray hits the target entity first, or hits nothing within `max_distance`.
/// * `false` if the ray hits another entity before the target.
/// 
/// # Notes
/// 
/// - This function does not handle multiple targets in a single ray.  
/// - If a different entity is hit first, you could trigger a message or event for auto-targeting.
fn has_line_of_sight(
    origin: Vec3,
    direction: Dir3,
    max_distance: f32,
    target: Entity,
    spatial_query: &impl RayCaster,
) -> bool {
    // If we hit something and it is NOT the target, the line of sight is blocked
    if let Some(hit_entity) = spatial_query.cast_ray_nearest(origin, direction, max_distance) {
        if hit_entity != target {
            // should spawn new Message with new target
            return false;
        }
    } 

    true
}

// // how to trigger
// fn player_input_fire(
//     keyboard: Res<Input<KeyCode>>,
//     mut fire_writer: EventWriter<WeaponFireRequest>,
//     query: Query<Entity, With<Weapon>>,
// ) {
//     if keyboard.just_pressed(KeyCode::Space) {
//         for weapon_entity in query.iter() {
//             fire_writer.send(WeaponFireRequest {
//                 weapon_entity,
//                 target_entity: None, // auto-target, or assign a specific target
//             });
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::{Dir3, Entity};

    // Mock SpatialQuery
    struct MockSpatialQuery {
        hit_entity: Option<Entity>,
    }

    impl MockSpatialQuery {
        fn new(hit_entity: Option<Entity>) -> Self {
            Self { hit_entity }
        }
    }

    impl RayCaster for MockSpatialQuery {
        fn cast_ray_nearest(
            &self,
            _origin: Vec3,
            _dir: Dir3,
            _max_distance: f32,
        ) -> Option<Entity> {
            self.hit_entity
        }
    }

    #[test]
    fn los_hits_target_returns_true() {
        let mut world = World::default();

        // Spawn entity to act as target
        let target = world.spawn_empty().id();

        let query = MockSpatialQuery::new(Some(target));
        let origin = Vec3::ZERO;
        let dir = Dir3::X;

        assert!(has_line_of_sight(origin, dir, 10.0, target, &query));
    }

    #[test]
    fn los_hits_other_returns_false() {
        let mut world = World::default();

        let target = world.spawn_empty().id();
        let blocker = world.spawn_empty().id();

        let query = MockSpatialQuery::new(Some(blocker));

        let origin = Vec3::ZERO;
        let dir = Dir3::X;

        assert!(!has_line_of_sight(origin, dir, 10.0, target, &query));
    }

    #[test]
    fn los_hits_nothing_returns_true() {
        let mut world = World::default();

        let target = world.spawn_empty().id();
        let query = MockSpatialQuery::new(None);

        let origin = Vec3::ZERO;
        let dir = Dir3::X;

        assert!(has_line_of_sight(origin, dir, 10.0, target, &query));
    }
}