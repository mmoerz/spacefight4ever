use bevy::prelude::*;
use avian3d::prelude::*;

use crate::game::combat::health::*;


fn raycast_damage_system(
    avian: Res<AvianWorld>,
    query: Query<(Entity, &Transform, &Collider)>,
    mut damage_writer: MessageWriter<HealthDamageReceived>,
) {
    let ray_origin = Vec2::new(0.0, 0.0); // example start
    let ray_dir = Vec2::new(1.0, 0.0).normalize();
    let ray_max_len = 100.0;

    let mut closest_hit: Option<(Entity, f32)> = None;

    for (entity, transform, collider) in query.iter() {
        if let Some(distance) = intersect_ray_collider(ray_origin, ray_dir, ray_max_len, transform.translation, collider) {
            if closest_hit.is_none() || distance < closest_hit.unwrap().1 {
                closest_hit = Some((entity, distance));
            }
        }
    }

    if let Some((hit_entity, distance)) = closest_hit {
        println!("Ray hit {:?} at distance {}", hit_entity, distance);
        damage_writer.write(HealthDamageReceived {
            entity: hit_entity,
            damage: 25.0,
            damage_efficiency: 1.0,
        });
    }
}

// Simple circle raycast intersection (for demo)
fn intersect_ray_collider(
    spatial_query: SpatialQuery,
    ray_origin: Vec3,
    ray_dir: Dir3,
    max_len: f32,
    solid: bool,
    filter: &SpatialQueryFilter
) -> Option<f32> {
    if let Some(first_hit) = spatial_query.cast_ray(ray_origin, ray_dir, max_len, solid, filter){
        
    }

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