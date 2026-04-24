use bevy::prelude::*;

use crate::game::player::playership::*;
use crate::game::ship::definitions::{
    module_definition::{ModuleDefinition, ModuleDefinitionIndex, ModuleId},
    ship_definition::{ShipModel, ShipDefinition, ShipDefinitionIndex},
    ship_models::{ShipModelIndex},
};
use crate::game::ship::modules::module::Module;
use crate::game::ship::modules::propulsion::compute_ship_capability;
use crate::ui::input::ship::spaceship_movement_system;
use crate::game::assets::GameState;

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<Assets<Gltf>>,
    def_assets: Res<Assets<ShipDefinition>>,
    model_index: Res<ShipModelIndex>,
    def_index: Res<ShipDefinitionIndex>,
    module_assets: Res<Assets<ModuleDefinition>>,
    module_index: Res<ModuleDefinitionIndex>,
) {
    let Some(prop_handle) = 
        module_index.get(&ModuleId("Ion Prop 1KN".to_string()))
        else { return; };
    spawn_player_ship_gltf(
        &mut commands, 
        ShipModel::Spitfire, assets, def_assets, model_index, def_index,
        vec![
            Module {
                id: 4,
                handle: prop_handle.clone(),
            },
            Module {
                id: 5,
                handle: prop_handle.clone(),
            },
        ],
        module_assets,
    );
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), (
                spawn_player.before(compute_ship_capability),
                compute_ship_capability,
            ))
            .add_systems(Update, spaceship_movement_system)
            //.add_systems(Update,  sync_visual_to_physics)
            ;
    }
}