use bevy::prelude::*;

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
            SceneRoot(self.model.clone()),
            Transform::from_xyz(0.0, 0.0, 0.0),
            GlobalTransform::default(),
            Visibility::Visible,
            OrbitCameraTarget,
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