use std::{
    collections::HashMap,
};

use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};
use serde::{Deserialize, Serialize};

use super::{
    load_error::AssetLoadError,
    weapon_definition::WeaponDefinition,
    shield_definition::ShieldDefinition,
    armor_definition::ArmorDefinition,
    support_definitions::SupportDefinition,
    propulsion_definition::PropulsionDefinition,
};

#[derive(Default,Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ModuleSize {
    #[default]
    Micro,
    Tiny,
    Small,
    Medium,
    Large,
    XLarge
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ModuleData {
    Propulsion(PropulsionDefinition),
    Weapon(WeaponDefinition),
    Shield(ShieldDefinition),
    Armor(ArmorDefinition),
    Support(SupportDefinition)
}

impl Default for ModuleData {
    fn default() -> Self {
        ModuleData::Support(SupportDefinition::Scan { strength: 0.0 })
    }
}

#[derive(Asset, TypePath, Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ModuleDefinition {
    pub name: String,
    pub kind: ModuleData,
    pub size: ModuleSize,
}

#[derive(Default, TypePath)]
pub struct ModuleDefinitionLoader;

impl AssetLoader for ModuleDefinitionLoader {
    type Asset = ModuleDefinition;
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

        let file: ModuleDefinition = ron::de::from_bytes(&bytes)?;
        Ok(file)
    }

    fn extensions(&self) -> &[&str] {
        &["ship.def.ron"]
    }
}

use bevy_asset_loader::asset_collection::AssetCollection;

// TODO: this is bevy_asset_loader specific, keep here or move to assets.rs?
#[derive(AssetCollection, Resource)]
pub struct ModuleDefinitions {
    #[asset(path = "data/modules", collection(typed))]
    folder: Vec<Handle<ModuleDefinition>>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct ModuleId(pub String);

#[derive(Resource, Default)]
pub struct ModuleDefinitionIndex {
    index: HashMap<ModuleId, Handle<ModuleDefinition>>,
}

impl ModuleDefinitionIndex {
    pub fn get(&self, id: &ModuleId) -> Option<&Handle<ModuleDefinition>> {
        self.index.get(id)
    }

    pub fn get_str(&self, name: &str) -> Option<&Handle<ModuleDefinition>> {
        self.index.get(&ModuleId(name.to_string()))
    }
}

pub fn build_index_once_system(
    defs: Res<Assets<ModuleDefinition>>,
    assets: Res<ModuleDefinitions>,
    mut index: ResMut<ModuleDefinitionIndex>,
) {
    for handle in &assets.folder {
        if let Some(def) = defs.get(handle) {
            debug!("Indexing module definition: {}", def.name);
            let id = ModuleId(def.name.clone());
            if index.index.insert(id, handle.clone()).is_some() {
                warn!("Duplicate module definition name: {}", def.name);
            } 
        }
    }
}