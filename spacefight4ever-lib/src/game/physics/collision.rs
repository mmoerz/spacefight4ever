/*
fn collision_damage_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut damage_writer: EventWriter<HealthDamageReceived>,
) {
    for event in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            // e.g., send damage based on collision impulse or object type
            damage_writer.send(HealthDamageReceived {
                entity: *e1,
                damage: 10.0,
                damage_profile: Default::default(),
                damage_efficiency: 1.0,
            });
        }
    }
}
    */