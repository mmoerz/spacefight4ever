use std::{
    str::FromStr,
    collections::HashMap,
};

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};
use serde::{Deserialize, Serialize};

use crate::game::ship::module::ModuleSize;
//use super::{definition_repository::NamedDefinition};
use super::load_error::AssetLoadError;
use super::ship_models::ShipModelIndex;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Deserialize, Serialize)]
pub enum ShipModel {
    #[default]
    Spitfire,
}

impl FromStr for ShipModel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Spitfire" => Ok(ShipModel::Spitfire),
            _ => Err(()),
        }
    }
}

impl ToString for ShipModel {
    fn to_string(&self) -> String {
        match self {
            ShipModel::Spitfire => "Spitfire".to_string(),
        }
    }
}

/// describes the static data of a weapon
#[derive(Asset, TypePath, Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ShipDefinition {
    //pub name: String,
    pub model: ShipModel,
    pub size: ModuleSize,
    pub mass: f32,
    pub max_cruise_speed: f32,
}

// impl NamedDefinition for ShipDefinition {
//     fn name(&self) -> &str {
//         &self.name
//     }
// }

#[derive(Default, TypePath)]
pub struct ShipDefinitionLoader;

impl AssetLoader for ShipDefinitionLoader {
    type Asset = ShipDefinition;
    type Settings = ();
    type Error = AssetLoadError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let file: ShipDefinition = ron::de::from_bytes(&bytes)?;
        Ok(file)
    }

    fn extensions(&self) -> &[&str] {
        &["ship.def.ron"]
    }
}

#[derive(Resource, Default)]
pub struct ShipDefinitionIndex {
    pub index: HashMap<ShipModel, Handle<ShipDefinition>>,
}

pub fn build_index_once_system(
    defs: Res<Assets<ShipDefinition>>,
    assets: Res<ShipDefinitions>,
    mut index: ResMut<ShipDefinitionIndex>,
    mut model_index: ResMut<ShipModelIndex>,
    asset_server: Res<AssetServer>,
) {
    for handle in &assets.folder {
        if let Some(def) = defs.get(handle) {
            println!("Indexing ship definition: {}", def.model.to_string());
            index.index.insert(def.model.clone(), handle.clone());
            let model_handle = asset_server.load(format!("ships/models/{}.glb", 
                    def.model.to_string()));
            model_index.index.insert(def.model.clone(), model_handle);
        }
    }
}

// fn update_index_on_change(
//     mut events: EventReader<AssetEvent<ShipDefinition>>,
//     defs: Res<Assets<ShipDefinition>>,
//     mut index: ResMut<ShipIndex>,
// ) {
//     for event in events.read() {
//         match event {
//             AssetEvent::Added { id }
//             | AssetEvent::Modified { id } => {
//                 if let Some(def) = defs.get(*id) {
//                     index.by_name.insert(def.name.clone(), Handle::weak(*id));
//                 }
//             }

//             AssetEvent::Removed { id } => {
//                 // remove stale entries
//                 index.by_name.retain(|_, h| h.id() != *id);
//             }
//         }
//     }
// }

pub struct ShipDefinitionPlugin;

impl Plugin for ShipDefinitionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<ShipDefinition>()
            .init_asset_loader::<ShipDefinitionLoader>()
            .init_resource::<ShipDefinitionIndex>()
            .add_systems(Startup, build_index_once_system);
    }
}

use bevy_asset_loader::asset_collection::AssetCollection;

// TODO: this is bevy_asset_loader specific, keep here or move to assets.rs?
#[derive(AssetCollection, Resource)]
pub struct ShipDefinitions {
    #[asset(path = "data/ships", collection(typed))]
    folder: Vec<Handle<ShipDefinition>>,
}

// /// just an alias to usize for ship ids
// ///
// /// usize is safer (no overflow), but it uses up i64 on 64bit systems
// /// u32 uses less memory, better for ECS
// pub type ShipDefinitionId = usize;

// /// Manager for ship definitions.
// /// 
// /// stores ship definitions in an immuteable Vec
// /// 
// /// ship defintions are retrieveable by id and name
// /// 
// /// name lookups use a hashmap for fast retrieval
// #[derive(Asset, TypePath)]
// pub struct ShipDefinitionRepository(pub DefinitionRepository<ShipDefinition>);

// impl ShipDefinitionRepository {
    
//     pub fn from_vec(defs: Vec<ShipDefinition>) -> Self {
//         Self(DefinitionRepository::from_vec(defs))
//     }

//     pub fn get_by_id(&self, id: ShipDefinitionId) -> &ShipDefinition {
//         self.0.get_by_id(id as usize)
//     }
//     pub fn get_by_name(&self, name: &str) -> Option<&ShipDefinition> {
//         self.0.get_by_name(name)
//     }
//     pub fn has_id(&self, id: ShipDefinitionId) -> bool {
//         self.0.has_id(id as usize)
//     }
//     pub fn has_name(&self, name: &str) -> bool {
//         self.0.has_name(name)
//     }
//     pub fn len(&self) -> usize {
//         self.0.len()
//     }
// }

// // custom asset loader for ButtonSkin, 
// // which reads from a RON file and loads the associated texture
// #[derive(Default, TypePath)]
// pub struct ShipDefinitionsLoader;

// // ship definition file structure
// #[derive(Asset, TypePath, Deserialize)]
// pub struct ShipDefinitionFile {
//     pub ships: Vec<ShipDefinition>,
// }

// // /// Implementation of the custom asset loader for `ButtonSkin`
// // /// This loader reads a RON file that specifies the texture atlas and mapping for button states,
// // /// and then loads the associated texture as a Bevy asset.
// impl AssetLoader for ShipDefinitionsLoader {
//     type Asset = ShipDefinitionFile;
//     type Settings = ();
//     type Error = AssetLoadError;
//     async fn load(
//         &self,
//         reader: &mut dyn Reader,
//         _settings: &(),
//         load_context: &mut LoadContext<'_>,
//     ) -> Result<Self::Asset, Self::Error> {
//         let mut bytes = Vec::new();
//         reader.read_to_end(&mut bytes).await?;
        
//         let file: ShipDefinitionFile = ron::de::from_bytes(&bytes)?;

//         for ship in file.ships {
//             load_context.add_labeled_asset(
//                 ship.name.clone(),
//                 ship,
//             );
//         }

//         Ok(file)
//     }

//     fn extensions(&self) -> &[&str] {
//         &["ship.def.ron"]
//     }
// }

