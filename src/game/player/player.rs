use bevy::prelude::*;

use crate::game::combat::{health::*, health_basetypes::Layered};
use super::ship::*;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Name::new("PlayerShip"),
        PlayerShip,
        ShipHealth {
            values: Layered { values: [3, 10, 20 ] },
            values_max: Layered { values: [10, 10, 20] },
        },
        Transform::default(),
    ));
}