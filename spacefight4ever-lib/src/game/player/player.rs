use bevy::prelude::*;

use crate::game::player::playership::*;
use crate::game::ship::definitions::{
    ship_definition::{ShipModel, ShipDefinition, ShipDefinitionIndex},
    ship_models::{ShipModelIndex},
};
//use crate::game::ship::module::{ModuleSize, HardPointType};
use crate::ui::input::ship::spaceship_movement_system;
use crate::game::assets::GameState;

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<Assets<Gltf>>,
    def_assets: Res<Assets<ShipDefinition>>,
    model_index: Res<ShipModelIndex>,
    def_index: Res<ShipDefinitionIndex>,
) {
    spawn_player_ship_gltf(&mut commands, ShipModel::Spitfire, assets, def_assets, model_index, def_index);
    //spawn_player_ship(&mut commands, Shipassets);
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