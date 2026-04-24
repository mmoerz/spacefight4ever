use bevy::prelude::*;
use avian3d::prelude::*;

use crate::game::combat::basetypes::*;
use crate::game::combat::health::*;
use crate::game::ship::definitions::ammunition_definition::{AmmunitionDefinitionRepository, AmmunitionDefinition};
use crate::game::ship::modules::module::Module;
use crate::game::ship::weapon::*;
use crate::game::ship::definitions::weapon_definition::*;



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

// i need a damage_system for environment damage

// weapons should only fire when there is a fire event
// i need a damage system for a specific weapon / weapongroup against a single target
// what i need to do is to check if the weapon has an angle limitation
// if it has I need to check wether the target is in range / optimal range
// if the target is within the angle (cone)
// depending on the weapon::Behaviour different checks need to be done:
//   * Beam *instant* damage: check if something is between shooter and target, if it is the first stuff inbetween eats the damage
//   * Missile spawns a missle that needs to fly to the target, will fly around anything inbetween
//   * Projectile spawns a projectile that needs to fly to the target, will also be eaten by anything inbetween, however needs to travel
//
// This is split here in:
// 1. Range check (cheap)
// 2. Angle / cone check (cheap)
// 3. Raycast / line-of-sight (expensive)
// 4. Apply damage / spawn missile or projectile
//
pub fn weapon_fire_system(
    mut commands: Commands,
    mut fire_events: MessageReader<WeaponFireRequest>,
    mut weapon_query: Query<(&Transform, &mut Weapon, &mut Ammunition)>,
    definition_query: Query<&WeaponBehavior>,
    target_query: Query<(Entity, &Transform), With<Target>>,
    spatial_query: SpatialQuery,
    mut damage_writer: MessageWriter<HealthDamageReceived>,
    weapon_repo: Res<Assets<ModuleDefinition>>,
    ammo_repo: Res<AmmunitionDefinitionRepository>,
    time: Res<Time>,
) {
    for event in fire_events.read() {
        let Ok((weapon_transform, mut weapon, mut ammunition)) = weapon_query.get_mut(event.weapon_entity) else { continue; };
        let Ok(behavior) = definition_query.get(event.weapon_entity) else { continue; };
        let weapon_def = weapon_repo.get_by_id(weapon.weapon_id);
        let ammo_def = ammo_repo.get_by_id(ammunition.ammo_id);

        if weapon.cooldown > 0.0 { continue; }
        if ammunition.count <= 0 { continue; }
        ammunition.count -= 1;
        weapon.cooldown = weapon_def.fire_rate;

        if let Some(target_entity) = event.target_entity {
            let Ok((_entity, target_transform)) = target_query.get(target_entity) else { continue; };

            let origin = weapon_transform.translation;
            let forward = weapon_transform.forward(); //rotation.mul_vec3(Vec3::X).normalize();
            let to_target = target_transform.translation - origin;
            let distance = to_target.length();
            let dir  = Dir3::new((to_target / distance).normalize()).unwrap();

            // range
            if distance > weapon_def.range[WeaponRangeType::Max] { continue; }

            // cone
            if let Some(max_angle) = weapon_def.max_angle {
                let cos_angle = forward.dot(dir.into()).clamp(-1.0, 1.0);
                if cos_angle < (max_angle * 0.5).cos() { continue; }
            }
            match behavior {
                WeaponBehavior::Beam => {
                    if has_line_of_sight(origin, dir, distance, target_entity, &spatial_query) { 
                    let raw_damage = compose_raw_damage(distance, weapon_def, ammo_def);

                    damage_writer.write(HealthDamageReceived {
                        target: target_entity,
                        damage: raw_damage, 
                        damage_profile: ammo_def.damage_profile,
                        damage_efficiency: ammo_def.damage_efficiency 
                    });
                }
                }
                WeaponBehavior::Missile => {
                    // TODO: no line of sight necessary however missiles must evade enemies on their way to the target
                    // TODO: spawn a missile with the target and the damage to apply
                    commands.spawn(Missile {
                        origin: origin,
                        target: target_entity,
                        base_damage: weapon_def.damage,
                        fuel: ammo_def.missile_fuel_max.unwrap_or(0) as f32,
                        ammo_id: ammunition.ammo_id,
                    });
                }
                WeaponBehavior::Projectile => {
                    // TODO: LOS necessary, but spawn a projectile since damage is not instantly applied
                    // TODO: include damage and target weapon behaviour
                    if has_line_of_sight(origin, dir, distance, target_entity, &spatial_query) {
                        // spawn projectile entity
                        commands.spawn(Projectile {
                            origin: origin,
                            target: target_entity,
                            base_damage: weapon_def.damage,
                            ammo_id: ammunition.ammo_id,
                        });
                    }
                }
            }
        }   
    }

    // Finally reduce cooldowns
    for (_transform, mut weapon, _ammo) in weapon_query.iter_mut() {
        weapon.cooldown = (weapon.cooldown - time.delta_secs()).max(0.0);
    }
}


/// pure math function for calculating damage
/// 
///
pub fn compose_raw_damage(
    distance: f32,
    weapon_def: &WeaponDefinition,
    ammo_def: &AmmunitionDefinition,
) -> i32 {
    let mut damage = weapon_def.damage + ammo_def.additional_damage;
    if distance <= weapon_def.range[WeaponRangeType::Min] || distance > weapon_def.range[WeaponRangeType::Max] {
        damage = 0.;
    } else if distance > weapon_def.range[WeaponRangeType::Optimal] {
        // reduce damage with the distance^2 from Optimal to max (and after it's 0)
        let over_optimal = distance - weapon_def.range[WeaponRangeType::Optimal];
        let optimal_max = weapon_def.range[WeaponRangeType::Max] - weapon_def.range[WeaponRangeType::Optimal];
        let reduction = 1.0 - over_optimal / optimal_max;
        damage *= reduction.powf(2.0);
    }
    
    damage as i32
}

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

// unittests for damage calculation
#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::ship::definitions::module_definition::ModuleSize;

    fn base_weapon() -> WeaponDefinition {
        let mut range = WeaponRange::default();
        range[WeaponRangeType::Min] = 10.0;
        range[WeaponRangeType::Optimal] = 50.0;
        range[WeaponRangeType::Max] = 100.0;

        WeaponDefinition { 
            behavior: WeaponBehavior::Beam, 
            range: range,
            max_angle: Some(360.0),
            damage: 100.0,
            fire_rate: 1.0,
            ammo_max: 10,
        }
    }

    fn base_ammo() -> AmmunitionDefinition {
        AmmunitionDefinition {
            name: "Lightning".into(),
            additional_damage: 20.0,
            damage_profile: Default::default(),
            damage_efficiency: Default::default(),
            range_modifier: 1.0,
            missile_fuel_max: Some(20),
        }
    }

    #[test]
    fn zero_damage_below_min_range() {
        let weapon = base_weapon();
        let ammo = base_ammo();

        let result = compose_raw_damage(5.0, &weapon, &ammo);

        assert_eq!(result, 0);
    }

    #[test]
    fn zero_damage_above_max_range() {
        let weapon = base_weapon();
        let ammo = base_ammo();

        let result = compose_raw_damage(150.0, &weapon, &ammo);

        assert_eq!(result, 0);
    }

    #[test]
    fn full_damage_within_optimal_range() {
        let weapon = base_weapon();
        let ammo = base_ammo();

        let result = compose_raw_damage(30.0, &weapon, &ammo);

        // 100 + 20
        assert_eq!(result, 120);
    }

    #[test]
    fn quadratic_falloff_after_optimal() {
        let weapon = base_weapon();
        let ammo = base_ammo();

        // halfway between optimal (50) and max (100)
        let result = compose_raw_damage( 75.0, &weapon, &ammo);

        // reduction = (1 - 0.5)^2 = 0.25
        // (100 + 20) * 0.25 = 30
        assert_eq!(result, 30);
    }

    #[test]
    fn damage_clamped_to_zero_when_reduction_negative() {
        let weapon = base_weapon();
        let ammo = base_ammo();

        // Force edge case
        let result = compose_raw_damage(100.0, &weapon, &ammo);

        // exactly at max should fall to zero
        assert_eq!(result, 0);
    }

    #[test]
    fn ammo_bonus_is_applied() {
        let mut weapon = base_weapon();
        weapon.damage = 50.0;

        let mut ammo = base_ammo();
        ammo.additional_damage = 25.0;

        let result = compose_raw_damage(30.0, &weapon, &ammo);

        assert_eq!(result, 75);
    }

    // raycasting unittests
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