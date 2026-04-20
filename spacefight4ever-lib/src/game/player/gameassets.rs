use bevy::prelude::*;

/// in order to properly load all resources before using the resource handles
/// a loading state prevents the game from running
#[derive(Default, States, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    InGame,
}

#[derive(Resource, Default)]
pub struct GameAssets {
    //pub player_ship: Handle<DynamicScene>,
    pub player_ship: Handle<Scene>
}

/// load all assets
fn load_assets(
    mut assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    //assets.player_ship = asset_server.load("ships/models/Spitfire.glb#Scene0");
    assets.player_ship = asset_server.load(GltfAssetLabel::Scene(0).from_asset("ships/models/Spitfire.glb"));
}

/// check if all assets are ready
fn check_assets_ready(
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    mut state: ResMut<NextState<GameState>>,
) {
    if !asset_server.is_loaded_with_dependencies(&assets.player_ship)
        || !asset_server.is_loaded_with_dependencies(&assets.player_ship) {
        return; // fast return if not ready
    }
    state.set(GameState::InGame);
}

fn debug_loading(
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
) {
    println!(
        "dynamic: {:?}, static: {:?}",
        asset_server.get_load_state(&assets.player_ship),
        asset_server.get_load_state(&assets.player_ship),
    );
}

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .init_resource::<GameAssets>()
            .add_systems(OnEnter(GameState::Loading), load_assets)
            .add_systems(Update, check_assets_ready.run_if(in_state(GameState::Loading)))
            .add_systems(Update, debug_loading.run_if(in_state(GameState::Loading)))
            ;
    }
}