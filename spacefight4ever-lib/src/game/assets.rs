use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::game::ship::definitions::{
    ship_definition::{
        ShipDefinition, ShipDefinitionIndex, 
        ShipDefinitionLoader, ShipDefinitions, 
        build_index_once_system
    },
    ship_models::{ ShipModels, ShipModelIndex },
};

// Asset handles (Bevy)
//  - textures
//  - models
//  - materials
// Game database (your system)
//  - ship definitions
//  - weapon stats
//  -factions
// Registry layer
//  - maps IDs → handles
//  - built once after loading
//
// ShipDefinition (game truth)
//     ↓
// ShipIndex (lookup layer)
//     ↓
// Handle<ShipDefinition> (asset reference)
//     ↓
// GPU / scene / material


/// in order to properly load all resources before using the resource handles
/// a loading state prevents the game from running
#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    AssetLoading,
    Indexing,
    //Loading,
    InGame,
}



///
pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .init_asset::<ShipDefinition>()
            .init_asset_loader::<ShipDefinitionLoader>()
            .init_resource::<ShipDefinitionIndex>()
            .init_resource::<ShipModelIndex>()

            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::Indexing)
                    .load_collection::<ShipDefinitions>()
                    .load_collection::<ShipModels>(),
            )
            .add_systems(OnEnter(GameState::Indexing), (
                build_index_once_system,
                next_state,
            ))
            ;
    }
}

fn next_state(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::InGame);
}