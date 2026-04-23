use std::{
    collections::HashMap,
};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use super::ship_definition::ShipModel;

#[derive(AssetCollection, Resource)]
pub struct ShipModels {
    #[asset(path = "ships/models", collection(typed))]
    pub ships: Vec<Handle<Gltf>>,
}

#[derive(Resource, Default)]
pub struct ShipModelIndex {
    pub index: HashMap<ShipModel, Handle<Gltf>>,
}

// pub fn build_index_once_system(
//     //mut commands: Commands,
//     defs: Res<Assets<Gltf>>,
//     assets: Res<ShipModels>,
//     mut index: ResMut<ShipModelIndex>,
// ) {
//     for handle in &assets.ships {
//         if let Some(def) = defs.get(handle) {
//             index.index.insert(def.model.clone(), handle.clone());
//         }
//     }
// }