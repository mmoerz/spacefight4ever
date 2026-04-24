use std::{
    str::FromStr,
    collections::HashMap,
};

use bevy::{
    //asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};
use serde::{Deserialize, Serialize};

use super::{
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
        ModuleData::Support {
            0: SupportDefinition::Scan { strength: 0.0 }
        }
    }
}

#[derive(Asset, TypePath, Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ModuleDefinition {
    pub name: String,
    pub kind: ModuleData,
    pub size: ModuleSize,
}



use bevy_asset_loader::asset_collection::AssetCollection;

// TODO: this is bevy_asset_loader specific, keep here or move to assets.rs?
#[derive(AssetCollection, Resource)]
pub struct ModuleDefinitions {
    #[asset(path = "data/modules", collection(typed))]
    folder: Vec<Handle<ModuleDefinition>>,
}

#[derive(Resource, Default)]
pub struct ModuleDefinitionIndex {
    pub index: HashMap<String, Handle<ModuleDefinition>>,
}

pub fn build_index_once_system(
    defs: Res<Assets<ModuleDefinition>>,
    assets: Res<ModuleDefinitions>,
    mut index: ResMut<ModuleDefinitionIndex>,
) {
    for handle in &assets.folder {
        if let Some(def) = defs.get(handle) {
            println!("Indexing module definition: {}", def.name);
            index.index.insert(def.name.clone(), handle.clone());
        }
    }
}