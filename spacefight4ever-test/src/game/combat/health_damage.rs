use bevy::prelude::*;

use spacefight4ever_lib::game::combat::health_basetypes::{HealthLayerType, HealthPercents};
use spacefight4ever_lib::game::combat::health::*;
use spacefight4ever_lib::game::combat::ships::ShipResistances;
use spacefight4ever_lib::prelude::combat::health_basetypes::LayeredHealth;

#[test]
fn test_simple_no_damage_apply_damage_system2() {
    let mut app = App::new();
    app.add_message::<HealthDamageReceived>();
    app.add_systems(Update, apply_damage_system);

    // spawn entity
    let entity = app.world_mut().spawn((
        ShipHealth {
            values: LayeredHealth { values: [10,5,20] },
            values_max: LayeredHealth { values: [10,5,20] }
        },
        ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0., 0., 0., 0.] }, // shield
                HealthPercents { values: [0., 0., 0., 0.] }, // armor
                HealthPercents { values: [0., 0., 0., 0.] }, // hull
            ],
        }),
    )).id();

    app.world_mut().write_message(HealthDamageReceived {
        entity,
        damage: 5,
        damage_profile: HealthPercents { values: [0.,1.,0.,0.] },
        damage_efficiency: DamageEfficiency { values: [
            HealthPercents { values: [1.,0.,0.,0.] },
            HealthPercents { values: [1.,0.,0.,0.] },
            HealthPercents { values: [1.,0.,0.,0.] },
        ] },
    });

    app.update();

    let health = app.world().get::<ShipHealth>(entity).unwrap();
    assert_eq!(health.values[HealthLayerType::Shield], 10);
}


#[test]
fn test_simple_apply_damage_system1() {
    let mut app = App::new();
    app.add_message::<HealthDamageReceived>();
    app.add_systems(Update, apply_damage_system);

    // spawn entity
    let entity = app.world_mut().spawn((
        ShipHealth {
            values: LayeredHealth { values: [10,5,20] },
            values_max: LayeredHealth { values: [10,5,20] }
        },
        ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0., 0., 0., 0.] }, // shield
                HealthPercents { values: [0., 0., 0., 0.] }, // armor
                HealthPercents { values: [0., 0., 0., 0.] }, // hull
            ],
        }),
    )).id();

    app.world_mut().write_message(HealthDamageReceived {
        entity,
        damage: 5,
        damage_profile: HealthPercents { values: [1.,0.,0.,0.] },
        damage_efficiency: DamageEfficiency { values: [
            HealthPercents { values: [1.,0.,0.,0.] },
            HealthPercents { values: [1.,0.,0.,0.] },
            HealthPercents { values: [1.,0.,0.,0.] },
        ] },
    });

    app.update();

    let health = app.world().get::<ShipHealth>(entity).unwrap();
    assert_eq!(health.values[HealthLayerType::Shield], 5);
}

#[test]
fn test_simple_apply_damage_system2() {
    let mut app = App::new();
    app.add_message::<HealthDamageReceived>();
    app.add_systems(Update, apply_damage_system);

    // spawn entity
    let entity = app.world_mut().spawn((
        ShipHealth {
            values: LayeredHealth { values: [10,5,20] },
            values_max: LayeredHealth { values: [10,5,20] }
        },
        ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0., 0., 0., 0.] }, // shield
                HealthPercents { values: [0., 0., 0., 0.] }, // armor
                HealthPercents { values: [0., 0., 0., 0.] }, // hull
            ],
        }),
    )).id();

    app.world_mut().write_message(HealthDamageReceived {
        entity,
        damage: 5,
        damage_profile: HealthPercents { values: [0.,1.,0.,0.] },
        damage_efficiency: DamageEfficiency { values: [
            HealthPercents { values: [0.,1.,0.,0.] },
            HealthPercents { values: [0.,1.,0.,0.] },
            HealthPercents { values: [0.,1.,0.,0.] },
        ] },
    });

    app.update();

    let health = app.world().get::<ShipHealth>(entity).unwrap();
    assert_eq!(health.values[HealthLayerType::Shield], 5);
}

#[test]
fn test_simple_apply_damage_system3() {
    let mut app = App::new();
    app.add_message::<HealthDamageReceived>();
    app.add_systems(Update, apply_damage_system);

    // spawn entity
    let entity = app.world_mut().spawn((
        ShipHealth {
            values: LayeredHealth { values: [10,5,20] },
            values_max: LayeredHealth { values: [10,5,20] }
        },
        ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0., 0., 0., 0.] }, // shield
                HealthPercents { values: [0., 0., 0., 0.] }, // armor
                HealthPercents { values: [0., 0., 0., 0.] }, // hull
            ],
        }),
    )).id();

    app.world_mut().write_message(HealthDamageReceived {
        entity,
        damage: 5,
        damage_profile: HealthPercents { values: [0.,0.,1.,0.] },
        damage_efficiency: DamageEfficiency { values: [
            HealthPercents { values: [0.,0.,1.,0.] },
            HealthPercents { values: [0.,0.,1.,0.] },
            HealthPercents { values: [0.,0.,1.,0.] },
        ] },
    });

    app.update();

    let health = app.world().get::<ShipHealth>(entity).unwrap();
    assert_eq!(health.values[HealthLayerType::Shield], 5);
}

#[test]
fn test_simple_apply_damage_system4() {
    let mut app = App::new();
    app.add_message::<HealthDamageReceived>();
    app.add_systems(Update, apply_damage_system);

    // spawn entity
    let entity = app.world_mut().spawn((
        ShipHealth {
            values: LayeredHealth { values: [10,5,20] },
            values_max: LayeredHealth { values: [10,5,20] }
        },
        ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0., 0., 0., 0.] }, // shield
                HealthPercents { values: [0., 0., 0., 0.] }, // armor
                HealthPercents { values: [0., 0., 0., 0.] }, // hull
            ],
        }),
    )).id();

    app.world_mut().write_message(HealthDamageReceived {
        entity,
        damage: 5,
        damage_profile: HealthPercents { values: [0.,0.,0.,1.] },
        damage_efficiency: DamageEfficiency { values: [
            HealthPercents { values: [1.,0.,0.,1.] },
            HealthPercents { values: [1.,0.,0.,1.] },
            HealthPercents { values: [1.,0.,0.,1.] },
        ] },
    });

    app.update();

    let health = app.world().get::<ShipHealth>(entity).unwrap();
    assert_eq!(health.values[HealthLayerType::Shield], 5);
}

#[test]
fn test_simple_with_resistence_apply_damage_system1() {
    let mut app = App::new();
    app.add_message::<HealthDamageReceived>();
    app.add_systems(Update, apply_damage_system);

    // spawn entity
    let entity = app.world_mut().spawn((
        ShipHealth {
            values: LayeredHealth { values: [10,5,20] },
            values_max: LayeredHealth { values: [10,5,20] }
        },
        ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0.5, 0., 0., 0.] }, // shield
                HealthPercents { values: [0., 0., 0., 0.] }, // armor
                HealthPercents { values: [0., 0., 0., 0.] }, // hull
            ],
        }),
    )).id();

    app.world_mut().write_message(HealthDamageReceived {
        entity,
        damage: 5,
        damage_profile: HealthPercents { values: [1.,0.,0.,0.] },
        damage_efficiency: DamageEfficiency { values: [
            HealthPercents { values: [1.,0.,0.,0.] },
            HealthPercents { values: [1.,0.,0.,0.] },
            HealthPercents { values: [1.,0.,0.,0.] },
        ] },
    });

    app.update();

    let health = app.world().get::<ShipHealth>(entity).unwrap();
    // actual computed floating point damage is 2.5, so in floatind point that would be 7.5
    // but since there is down rounding it is 7
    assert_eq!(health.values[HealthLayerType::Shield], 7); 
}

#[test]
fn test_simple_with_efficiency_apply_damage_system1() {
    let mut app = App::new();
    app.add_message::<HealthDamageReceived>();
    app.add_systems(Update, apply_damage_system);

    // spawn entity
    let entity = app.world_mut().spawn((
        ShipHealth {
            values: LayeredHealth { values: [10,5,20] },
            values_max: LayeredHealth { values: [10,5,20] }
        },
        ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0.1, 0., 0., 0.] }, // shield
                HealthPercents { values: [0., 0., 0., 0.] }, // armor
                HealthPercents { values: [0., 0., 0., 0.] }, // hull
            ],
        }),
    )).id();

    app.world_mut().write_message(HealthDamageReceived {
        entity,
        damage: 20,
        damage_profile: HealthPercents { values: [1.,0.,0.,0.] },
        damage_efficiency: DamageEfficiency { values: [
            HealthPercents { values: [0.2,0.,0.,0.] },
            HealthPercents { values: [1.,0.,0.,0.] },
            HealthPercents { values: [1.,0.,0.,0.] },
        ] },
    });

    app.update();

    let health = app.world().get::<ShipHealth>(entity).unwrap();
    assert_eq!(health.values[HealthLayerType::Shield], 6.4 as i32); // actually 6.4
}


#[test]
fn test_simple_with_resistence_and_efficiency_apply_damage_system1() {
    let mut app = App::new();
    app.add_message::<HealthDamageReceived>();
    app.add_systems(Update, apply_damage_system);

    // spawn entity
    let entity = app.world_mut().spawn((
        ShipHealth {
            values: LayeredHealth { values: [10,5,20] },
            values_max: LayeredHealth { values: [10,5,20] }
        },
        ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0.5, 0., 0., 0.] }, // shield
                HealthPercents { values: [0., 0., 0., 0.] }, // armor
                HealthPercents { values: [0., 0., 0., 0.] }, // hull
            ],
        }),
    )).id();

    app.world_mut().write_message(HealthDamageReceived {
        entity,
        damage: 20,
        damage_profile: HealthPercents { values: [1.,0.,0.,0.] },
        damage_efficiency: DamageEfficiency { values: [
            HealthPercents { values: [0.2,0.,0.,0.] },
            HealthPercents { values: [1.,0.,0.,0.] },
            HealthPercents { values: [1.,0.,0.,0.] },
        ] },
    });

    app.update();

    let health = app.world().get::<ShipHealth>(entity).unwrap();
    assert_eq!(health.values[HealthLayerType::Shield], 8);
}