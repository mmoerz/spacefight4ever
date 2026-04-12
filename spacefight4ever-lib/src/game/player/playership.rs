use bevy::prelude::*;
use avian3d::prelude::*;

use crate::ui::camera::{OrbitCameraTarget};
use crate::game::player::gameassets::*;

#[derive(Component)]
pub struct PlayerShip;

pub struct PlayerShipBuilder {
    model: Handle<Scene>,
}

impl PlayerShipBuilder {
    pub fn new(
        model: Handle<Scene>,
    ) -> Self {
        Self {
            model,
        }
    }

    // TODO:avian3d collision box is missing here
    // not visible, could be problem that the model is not loaded yet, but should be visible once loaded
    pub fn build(self) -> impl Bundle {
        (
            PlayerShip,
            
            // avian3d
            RigidBody::Dynamic,
            Collider::capsule(1.0, 2.5),
            //ConstantForce::new(0., 0., 0.),
            GravityScale(0.0),

            Transform::from_xyz(0.0, 0.0, 0.0),
            Visibility::Visible,

            OrbitCameraTarget,
            
            children![(
            (
                SceneRoot(self.model.clone()),
                Transform::IDENTITY,
            ),
            (
                Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
                MeshMaterial3d(materials.add(Color::RED)),
                Transform::from_xyz(0.0, 0.0, 0.0),
            )
            )],        
        )
    }
}

pub fn spawn_player_ship(
    commands: &mut Commands,
    assets: &GameAssets,
) -> Entity {
    commands.spawn(
        PlayerShipBuilder::new(assets.player_ship.clone()).build()
    ).id()
}

pub fn player_ship_bundle(
    assets: &GameAssets,
) -> impl Bundle {
    PlayerShipBuilder::new(assets.player_ship.clone()).build()
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

pub trait ChildButtonSpawner {
    fn spawn_player_ship(
        &mut self,
        model: Handle<Scene>,
    ) -> Entity;
}

impl ChildButtonSpawner for ChildSpawnerCommands<'_> {
    fn spawn_player_ship(
        &mut self,
        model: Handle<Scene>,
    ) -> Entity {
        self.spawn(
            PlayerShipBuilder::new(model).build()
        ).id()
    }
}