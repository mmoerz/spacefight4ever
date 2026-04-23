use bevy::prelude::*;
use avian3d::prelude::*;
use spacefight4ever_ui::ui::assets;

use crate::ui::input::ship::SpaceshipController;
use crate::game::ship::weapon::{Weapon, Ammunition};
use crate::game::ship::definitions::{
    ship_definition::{ShipModel, ShipDefinition, },
    ship_models::{ShipModelIndex, ShipModels},
};
use crate::game::ship::module::{ModuleSize, HardPointType};
use crate::game::{combat::{health::*, health_basetypes::LayeredHealth}, ship::{bundle::WeaponModuleBundle, module::{Module, MountPoint, MountType}}};

use crate::ui::camera::{OrbitCameraTarget};

#[derive(Component)]
pub struct PlayerShip;

pub struct PlayerShipBuilder {
    ship_model: ShipModel,
    model: Handle<Scene>,
}

impl PlayerShipBuilder {
    pub fn new(
        ship_model: ShipModel,
        model: Handle<Scene>,
    ) -> Self {
        Self {
            ship_model,
            model,
        }
    }

    // TODO:avian3d collision box is missing here
    // not visible, could be problem that the model is not loaded yet, but should be visible once loaded
    pub fn build(
        self,
        commands: &mut Commands,
) -> Entity {
        let ship_id = commands.spawn((
            Name::new("PlayerShip"),
            PlayerShip,
            SpaceshipController::default(),
            self.ship_model,

            ShipHealth {
                values: LayeredHealth { values: [3, 10, 20 ] },
                values_max: LayeredHealth { values: [10, 10, 20] },
            },
            
            // avian3d
            RigidBody::Dynamic,
            //Collider::capsule(1.0, 2.5),
            Collider::sphere(1.5),
            LinearDamping(0.5), 
            AngularDamping(1.0), 
            //ConstantForce::new(0., 0., 0.),
            GravityScale(0.0),

            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::Visible,

            OrbitCameraTarget,
        )).with_children(|ship| {
            ship.spawn((
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
            ));
        }).id();

        let model_id = commands.spawn((
            SceneRoot(self.model.clone()),
            //Transform::from_xyz(0.0, 0.0, 0.0),
            // Transform::from_rotation(
            //     Quat::from_rotation_y(std::f32::consts::FRAC_2_PI) // 90° Y rotation
            // ),
            Visibility::Visible,
            Name::new("SceneRoot"),
        )).id();

        commands.entity(ship_id).add_child(model_id);

        ship_id
    }
}

pub fn spawn_player_ship(
    commands: &mut Commands,
    model: ShipModel,
    scene: Handle<Scene>,
) -> Entity {
    PlayerShipBuilder::new(model, scene)
        .build(commands)
    //Entity::PLACEHOLDER
}

pub fn spawn_player_ship_gltf(
    commands: &mut Commands,
    model: ShipModel,
    assets: Res<Assets<Gltf>>,
    model_assets: Res<ShipModelIndex>,
    //def_assets: Res<ShipDefinitionIndex>,
) -> Entity {
    let handle = model_assets.index.get(&model).unwrap();
    let ship_model = assets.get(handle).unwrap();
    let scene  = ship_model.scenes[0].clone();
    //let def_handle = def_assets.index.get(&model).unwrap();

    spawn_player_ship(commands, model, scene)
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

// pub trait ChildButtonSpawner {
//     fn spawn_player_ship(
//         &mut self,
//         model: Handle<Scene>,
//     ) -> Entity;
// }

// impl ChildButtonSpawner for ChildSpawnerCommands<'_> {
//     fn spawn_player_ship(
//         &mut self,
//         model: Handle<Scene>,
//     ) -> Entity {
//         self.spawn(
//             PlayerShipBuilder::new(model).build()
//         ).id()
//     }
// }

// =======
// physics injector
// =======

// yeah, this is fucking great, scene is some sort of magic node that behaves
// outside of expectations, creates it's own static unmutable little universe
// so from the outside there  is no way to influence the mesh, we need to 
// crawl inside the gltf model and stick tack physics onto the meshes
// fn inject_fucking_physics(
//     mut commands: Commands,
//     // We need to wait for the scene to load, so we use added query
//     mut scenes: Query<(Entity, &Children), Added<SceneInstance>>,
// ) {
//     for (parent_entity, children) in scenes.iter_mut() {
//         // Find the mesh entity within the glTF hierarchy
//         // This usually requires traversing children or checking names
//         for &child in children.iter() {
//             // Apply RigidBody and Collider to the visual mesh
//             commands.entity(child)
//                 .insert(RigidBody::Dynamic)
//                 .insert(Collider::convex_hull_from_mesh(&child_mesh).unwrap()) // Simplified
//                 .insert(MassPropertiesBundle::from_center_of_mass(Vec3::ZERO, 1.0));
//         }
//     }
// }

// sync the model to the physics
// pub fn sync_visual_to_physics(
//     q: Query<(&GlobalTransform, &Children), With<PlayerShip>>,
//     mut transforms: Query<&mut Transform>,
// ) {
//     for (gt, children) in &q {
//         for &child in children {
//             if let Ok(mut t) = transforms.get_mut(child) {
//                 t.translation = gt.translation();
//             }
//         }
//     }
// }