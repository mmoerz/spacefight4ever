use bevy::prelude::*;

use crate::game::{combat::{health_basetypes::LayeredHealth}, ship::{bundle::WeaponModuleBundle, module::{Module, MountPoint, MountType}}};
//use crate::game::ship::weapon::{Weapon, Ammunition};
use crate::game::player::playership::*;
//use crate::game::ship::module::{ModuleSize, HardPointType};
use crate::ui::input::ship::spaceship_movement_system;
use crate::game::assets::{GameState, GameAssets};

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<Assets<Gltf>>,
    assets_collection: Res<GameAssets>,
) {
    let ship_model = assets.get(&assets_collection.player_ship).unwrap();
    let scene  = ship_model.scenes[0].clone();

    spawn_player_ship(&mut commands, scene);
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