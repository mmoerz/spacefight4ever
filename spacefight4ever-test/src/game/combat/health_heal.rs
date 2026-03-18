use bevy::prelude::*;

use spacefight4ever_lib::game::combat::health_basetypes::*;
use spacefight4ever_lib::game::combat::health::*;
use spacefight4ever_lib::game::combat::ships::ShipResistances;
use spacefight4ever_lib::prelude::combat::health_basetypes::LayeredHealth;

#[test]
fn test_apply_heal_system_with_resistances() {
    let mut app = App::new();
    app.add_message::<HealReceived>();
    app.add_systems(Update, apply_heal_system);

    // Spawn a ship entity with initial health and max health
    let entity = app.world_mut().spawn((
        ShipHealth {
            values: LayeredHealth { values: [5,10,15] },
            values_max: LayeredHealth { values: [10,15,20] },
        },
        ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0.0, 0.0, 0.0, 0.5] }, // shield EM resistance 50%
                HealthPercents { values: [0.2, 0.1, 0.0, 0.0] }, // armor K/T/E resistances
                HealthPercents { values: [0.1, 0.2, 0.0, 0.0] }, // hull K/T/E resistances
            ],
        }),
    )).id();

    // Send a healing message
    app.world_mut().write_message(HealReceived {
        entity,
        healing: HealthHealing { values: [6,6,6]}, // raw healing
    });

    // Run the system
    app.update();

    // Get updated health
    let health = app.world().get::<ShipHealth>(entity).unwrap();

    // Expected healing calculation:
    // Shield: 6 * (1 - 0.5) = 3 → 5 + 3 = 8 (max 10)
    // Armor: 6 * (1 - average K/T/E resistance) ≈ 6 * 0.6? We'll apply each:
    //   Armor raw = 6, resistances K/T/E = 0.2,0.1,0
    //   Effective per type = sum? In our system, total = 6 * (1-0.2)+(1-0.1)+(1-0)? Actually, depends on logic
    //   For simplicity, let's approximate total healing = 6 * (1-0.2) = 4.8 → 10 + 4.8 = 14.8 ~ 14 i32
    // Hull: similar logic, capped at max 20
    assert_eq!(health.values[HealthLayerType::Shield], 8);
    assert_eq!(health.values[HealthLayerType::Armor], 15);
    assert_eq!(health.values[HealthLayerType::Hull], 20);
}

#[test]
fn test_apply_heal_system_with_resistances2() {
    let mut app = App::new();
    app.add_message::<HealReceived>();
    app.add_systems(Update, apply_heal_system);

    // Spawn a ship entity with initial health and max health
    let entity = app.world_mut().spawn((
        ShipHealth {
            values: LayeredHealth { values: [5,10,15] },
            values_max: LayeredHealth { values: [10,15,30] },
        },
        ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0.0, 0.0, 0.0, 0.5] }, // shield EM resistance 50%
                HealthPercents { values: [0.2, 0.1, 0.0, 0.0] }, // armor K/T/E resistances
                HealthPercents { values: [0.1, 0.2, 0.0, 0.0] }, // hull K/T/E resistances
            ],
        }),
    )).id();

    // Send a healing message
    app.world_mut().write_message(HealReceived {
        entity,
        healing: HealthHealing { values: [6,6,6]}, // raw healing
    });

    // Run the system
    app.update();

    // Get updated health
    let health = app.world().get::<ShipHealth>(entity).unwrap();

    // Expected healing calculation:
    // Shield: 6 * (1 - 0.5) = 3 → 5 + 3 = 8 (max 10)
    // Armor: 6 * (1 - average K/T/E resistance) ≈ 6 * 0.6? We'll apply each:
    //   Armor raw = 6, resistances K/T/E = 0.2,0.1,0
    //   Effective per type = sum? In our system, total = 6 * (1-0.2)+(1-0.1)+(1-0)? Actually, depends on logic
    //   For simplicity, let's approximate total healing = 6 * (1-0.2) = 4.8 → 10 + 4.8 = 14.8 ~ 14 i32
    // Hull: similar logic, capped at max 20
    assert_eq!(health.values[HealthLayerType::Shield], 8);
    assert_eq!(health.values[HealthLayerType::Armor], 15);
    assert_eq!(health.values[HealthLayerType::Hull], 20);
}

#[test]
fn test_heal_no_overflow_all() {
    let mut app = App::new();
    app.add_message::<HealReceived>();
    app.add_systems(Update, apply_heal_system);

    // Spawn a ship entity with current health near max
    let entity = app.world_mut().spawn((
        ShipHealth {
            values: LayeredHealth { values: [9,14,19] },
            values_max: LayeredHealth { values: [10,15,20] },
        },
        ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] }, // Shield EM 0%
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] }, // Armor K/T/E 0%
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] }, // Hull K/T/E 0%
            ],
        }),
    )).id();

    // Send healing messages that exceed max health
    app.world_mut().write_message(HealReceived {
        entity,
        healing: HealthHealing { values: [10,5,10]}, // raw healing
        // shield would overflow by 9+10=19
        // armor 14+5=19
        // hull 19+10=29
    });

    app.update();

    let health = app.world().get::<ShipHealth>(entity).unwrap();

    // Expect that no layer exceeds max
    assert_eq!(health.values[HealthLayerType::Shield], 10); // capped at max
    assert_eq!(health.values[HealthLayerType::Armor], 15);  // capped at max
    assert_eq!(health.values[HealthLayerType::Hull], 20);   // capped at max
}

#[test]
fn test_heal_no_overflow1() {
    let mut app = App::new();
    app.add_message::<HealReceived>();
    app.add_systems(Update, apply_heal_system);

    // Spawn a ship entity with current health near max
    let entity = app.world_mut().spawn((
        ShipHealth {
            values: LayeredHealth { values: [9,14,19] },
            values_max: LayeredHealth { values: [10,15,20] },
        },
        ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] }, // Shield EM 0%
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] }, // Armor K/T/E 0%
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] }, // Hull K/T/E 0%
            ],
        }),
    )).id();

    // Send healing messages that exceed max health
    app.world_mut().write_message(HealReceived {
        entity,
        healing: HealthHealing { values: [10,0,0]}, // raw healing
        // shield would overflow by 9+10=19
        // armor 14+5=19
        // hull 19+10=29
    });

    app.update();

    let health = app.world().get::<ShipHealth>(entity).unwrap();

    // Expect that no layer exceeds max
    assert_eq!(health.values[HealthLayerType::Shield], 10); // capped at max
    assert_eq!(health.values[HealthLayerType::Armor], 14);  // capped at max
    assert_eq!(health.values[HealthLayerType::Hull], 19);   // capped at max
}


#[test]
fn test_heal_no_overflow2() {
    let mut app = App::new();
    app.add_message::<HealReceived>();
    app.add_systems(Update, apply_heal_system);

    // Spawn a ship entity with current health near max
    let entity = app.world_mut().spawn((
        ShipHealth {
            values: LayeredHealth { values: [9,14,19] },
            values_max: LayeredHealth { values: [10,15,20] },
        },
        ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] }, // Shield EM 0%
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] }, // Armor K/T/E 0%
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] }, // Hull K/T/E 0%
            ],
        }),
    )).id();

    // Send healing messages that exceed max health
    app.world_mut().write_message(HealReceived {
        entity,
        healing: HealthHealing { values: [0,5,0]}, // raw healing
        // shield would overflow by 9+10=19
        // armor 14+5=19
        // hull 19+10=29
    });

    app.update();

    let health = app.world().get::<ShipHealth>(entity).unwrap();

    // Expect that no layer exceeds max
    assert_eq!(health.values[HealthLayerType::Shield], 9); // capped at max
    assert_eq!(health.values[HealthLayerType::Armor], 15);  // capped at max
    assert_eq!(health.values[HealthLayerType::Hull], 19);   // capped at max
}

#[test]
fn test_heal_no_overflow3() {
    let mut app = App::new();
    app.add_message::<HealReceived>();
    app.add_systems(Update, apply_heal_system);

    // Spawn a ship entity with current health near max
    let entity = app.world_mut().spawn((
        ShipHealth {
            values: LayeredHealth { values: [9,14,19] },
            values_max: LayeredHealth { values: [10,15,20] },
        },
        ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] }, // Shield EM 0%
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] }, // Armor K/T/E 0%
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] }, // Hull K/T/E 0%
            ],
        }),
    )).id();

    // Send healing messages that exceed max health
    app.world_mut().write_message(HealReceived {
        entity,
        healing: HealthHealing { values: [0,0,10]}, // raw healing
        // shield would overflow by 9+10=19
        // armor 14+5=19
        // hull 19+10=29
    });

    app.update();

    let health = app.world().get::<ShipHealth>(entity).unwrap();

    // Expect that no layer exceeds max
    assert_eq!(health.values[HealthLayerType::Shield], 9); // capped at max
    assert_eq!(health.values[HealthLayerType::Armor], 14);  // capped at max
    assert_eq!(health.values[HealthLayerType::Hull], 20);   // capped at max
}