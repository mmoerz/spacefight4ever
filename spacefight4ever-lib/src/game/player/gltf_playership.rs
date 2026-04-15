use bevy::prelude::*;
use avian3d::prelude::*;

use super::playership::*;

use crate::game::{
    combat::{health::*, health_basetypes::LayeredHealth},
    player::gameassets::GameAssets,
    ship::{
        bundle::WeaponModuleBundle,
        module::{Module, MountPoint, MountType, ModuleSize, HardPointType},
        weapon::{Weapon, Ammunition}
    }
};

pub struct GltfPlayershipBuilder {
    model: Handle<DynamicScene>,
}

impl GltfPlayershipBuilder {
    pub fn new(
        model: Handle<DynamicScene>,
    ) -> Self {
        Self {
            model,
        }
    }

    pub fn build(
        self,
) -> impl Bundle {
        (
            Name::new("PlayerShip"),

            PlayerShip,
            ShipHealth {
                values: LayeredHealth { values: [3, 10, 20 ] },
                values_max: LayeredHealth { values: [10, 10, 20] },
            },
            
            // avian3d
            RigidBody::Dynamic,

            DynamicSceneRoot(self.model.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::Visible,
            children![(
                Name::new("Mountpoint"),
                MountPoint {
                    id: 0,
                    kind: MountType::Hardpoint(HardPointType::Weapon),
                    allowed_size: ModuleSize::Small,
                },
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
                    },
                ),
            )]
        )
    }
}

pub fn spawn_player_ship(
    commands: &mut Commands,
    assets: &GameAssets,
) -> Entity {
    commands.spawn(
    GltfPlayershipBuilder::new(assets.player_ship.clone())
            .build()
    ).id()
}

pub  fn player_ship_bundle(
    assets: &GameAssets,
) -> impl Bundle
{
    GltfPlayershipBuilder::new(assets.player_ship.clone())
        .build()
}