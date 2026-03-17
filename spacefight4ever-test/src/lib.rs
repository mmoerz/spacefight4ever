use bevy::prelude::*;

use spacefight4ever_lib::game::combat::health::*;
use spacefight4ever_lib::game::combat::health_basetypes::{HealthLayerType, HealthPercents};
use spacefight4ever_lib::game::combat::ships::ShipResistances;
use spacefight4ever_lib::prelude::combat::health_basetypes::Layered;

#[test]
fn test_apply_damage_system() {
    let mut app = App::new();
    app.add_message::<HealthDamageReceived>();
    app.add_systems(Update, apply_damage_system);

    // spawn entity
    let entity = app.world_mut().spawn((
        ShipHealth {
            values: Layered { values: [10,5,20] },
            values_max: Layered { values: [10,5,20] }
        },
        ShipResistances(Layered {
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