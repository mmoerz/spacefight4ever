use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::game::ship::definitions::ship_definition::{
    ShipAssets, ShipDefinition, ShipDefinitionLoader, ShipDefinitionIndex,
    build_index_once_system
};

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

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "ships/models/Spitfire.glb")]
    pub player_ship: Handle<Gltf>,
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
            .add_loading_state(
                LoadingState::new(GameState::AssetLoading)
                    .continue_to_state(GameState::InGame)
                    .load_collection::<ShipAssets>()
                    .load_collection::<GameAssets>(),
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