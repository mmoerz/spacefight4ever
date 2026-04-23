use bevy::prelude::*;

use crate::game::{combat::{health_basetypes::LayeredHealth}, ship::{bundle::WeaponModuleBundle, module::{Module, MountPoint, MountType}}};
//use crate::game::ship::weapon::{Weapon, Ammunition};
use crate::game::player::playership::*;
use crate::game::ship::definitions::{
    ship_definition::{ShipModel, ShipDefinition, },
    ship_models::{ShipModelIndex, ShipModels},
};
//use crate::game::ship::module::{ModuleSize, HardPointType};
use crate::ui::input::ship::spaceship_movement_system;
use crate::game::assets::GameState;

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<Assets<Gltf>>,
    model_assets: Res<ShipModelIndex>,
) {
    spawn_player_ship_gltf(&mut commands, ShipModel::Spitfire, assets, model_assets);
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), spawn_player)
            .add_systems(Update, spaceship_movement_system)
            //.add_systems(Update,  sync_visual_to_physics)
            ;
    }
}