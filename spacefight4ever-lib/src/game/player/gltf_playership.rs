use bevy::{
    gltf::GltfExtras,
    prelude::*,
};
use avian3d::prelude::*;
use serde::Deserialize;

use super::playership::*;

use crate::{game::{
    combat::{health::*, health_basetypes::LayeredHealth},
    player::gameassets::GameAssets,
    ship::{
        bundle::WeaponModuleBundle,
        module::{HardPointType, Module, ModuleSize, MountPoint, MountType},
        weapon::{Ammunition, Weapon}
    }
}, ui::camera::OrbitCameraTarget};

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
            OrbitCameraTarget,
            
            // avian3d
            RigidBody::Dynamic,
            GravityScale(0.0),

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

#[derive(Deserialize)]
struct MyColliderExtras {
    collider_type: String,
}

fn on_gltf_extras_added(
    trigger: On<Add, GltfExtras>,
    mut commands: Commands,
    query: Query<(&GltfExtras, &Mesh3d)>,
    meshes: Res<Assets<Mesh>>,
) {
    let entity = trigger.observer().entity();
    if let Ok((extras, mesh_3d)) = query.get(entity) {
        let mesh_handle = &mesh_3d.0;
        if let Ok(data) = serde_json::from_str::<MyColliderExtras>(&extras.value) {
            if let Some(mesh) = meshes.get(mesh_handle) {
                match data.collider_type.as_str() {
                    "convex" => {
                        if let Some(c) = Collider::convex_hull_from_mesh(mesh) {
                            commands.entity(entity).insert(c);
                        }
                    }
                    "trimesh" => {
                        if let Some(c) = Collider::trimesh_from_mesh(mesh) {
                            commands.entity(entity).insert(c);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}


pub fn spawn_player(
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    spawn_player_ship(&mut commands, &assets);
}

pub struct GltfPlayerShipPlugin;

impl Plugin for GltfPlayerShipPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_observer(on_gltf_extras_added)
            //.add_systems(Update, sync_visual_to_physics)
            ;
    }
}

//