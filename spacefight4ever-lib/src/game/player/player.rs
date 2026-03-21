use bevy::prelude::*;

use crate::game::{combat::{health::*, health_basetypes::LayeredHealth}, ship::{bundle::WeaponModuleBundle, module::{Module, MountPoint, MountType}}};
use crate::game::ship::weapon::{Weapon, Ammunition};
use crate::game::player::ship::PlayerShip;
use crate::game::ship::module::{ModuleSize, HardPointType};



pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Name::new("PlayerShip"),
        PlayerShip,
        ShipHealth {
            values: LayeredHealth { values: [3, 10, 20 ] },
            values_max: LayeredHealth { values: [10, 10, 20] },
        },
        Transform::default(),
    )).with_children(|ship| {
        ship.spawn(
            MountPoint {
                id: 0,
                kind: MountType::Hardpoint(HardPointType::Weapon),
                allowed_size: ModuleSize::Small,
            })
            .with_children(|mountpoint| {
                mountpoint.spawn(
                WeaponModuleBundle::new(
                        Module {
                            id: 0,
                            name: "Foobar".into(),
                            size: ModuleSize::Small,
                            kind: MountType::Hardpoint(HardPointType::Weapon),
                        },
                        Weapon {
                            weapon_id: 0,
                            cooldown: 10.0,
                        },
                        Ammunition{
                            ammo_id: 0,
                            count: 10,
                        }
                    ));
            });
    });
}