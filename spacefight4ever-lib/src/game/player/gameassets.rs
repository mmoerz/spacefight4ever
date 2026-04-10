use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameAssets {
    pub player_ship: Handle<Scene>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(GameAssets {
        player_ship: asset_server.load("ships/models/Spitfire.glb#Scene0"),
    });
}

pub struct GameAssetsPlugin;

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameAssets>()
            .add_systems(Startup, load_assets);
    }
}