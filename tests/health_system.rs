// use bevy::prelude::*;
// use spacefight4ever::game::combat::health::*;

// #[test]
// fn test_apply_damage_vector() {
//     let mut app = App::new();
//     app.add_system(apply_damage_system);

//     let entity = app.world.spawn((
//         ShipHealth { shield: 10, shield_max: 10, armor: 5, armor_max: 5, hull: 20, hull_max: 20 },
//     )).id();

//     // Send a damage event
//     app.world.send_message(HealthDamageReceived { entity, damage: 5, damage_type: DamageType::Kinetic });

//     app.update(); // run one frame

//     let health = app.world.get::<ShipHealth>(entity).unwrap();
//     assert_eq!(health.shield, 5); // shield should absorb first
// }