use std::sync::OnceState;

use bevy::prelude::*;

use crate::game::{combat::{health::*, health_basetypes::LayeredHealth}, ship::{bundle::WeaponModuleBundle, module::{Module, MountPoint, MountType}}};
use crate::game::ship::weapon::{Weapon, Ammunition};
use crate::game::player::{playership::*, gameassets::GameAssets};
use crate::game::ship::module::{ModuleSize, HardPointType};
//use crate::game::player::playership::sync_visual_to_physics;
use crate::game::player::gameassets::GameState;

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    // commands.spawn((
    //     Name::new("PlayerShip"),
    //     player_ship_bundle(&assets),
    //     // Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    //     // MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
    //     ShipHealth {
    //         values: LayeredHealth { values: [3, 10, 20 ] },
    //         values_max: LayeredHealth { values: [10, 10, 20] },
    //     },
    // )).with_children(|ship| {
    //     ship.spawn((
    //         MountPoint {
    //             id: 0,
    //             kind: MountType::Hardpoint(HardPointType::Weapon),
    //             allowed_size: ModuleSize::Small,
    //         },
    //         WeaponModuleBundle::new(
    //             Module {
    //                 id: 0,
    //                 name: "Foobar".into(),
    //                 size: ModuleSize::Small,
    //                 kind: MountType::Hardpoint(HardPointType::Weapon),
    //             },
    //             Weapon {
    //                 weapon_id: 0,
    //                 cooldown: 10.0,
    //             },
    //             Ammunition{
    //                 ammo_id: 0,
    //                 count: 10,
    //             },
    //         ),
    //     ));
    // });
    spawn_player_ship(&mut commands, &assets);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), spawn_player)
            //.add_systems(Update,  sync_visual_to_physics)
            ;
    }
}