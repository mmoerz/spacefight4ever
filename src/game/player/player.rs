use bevy::prelude::*;

use crate::game::combat::health::*;
use super::ship::*;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Name::new("PlayerShip"),
        PlayerShip,
        ShipHealth {
            shield: 3,
            shield_max: 3,
            armor: 10,
            armor_max: 10,
            hull: 20,
            hull_max: 20,
        },
        Transform::default(),
    ));
}