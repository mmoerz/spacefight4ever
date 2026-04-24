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

use crate::game::ship::{
    definitions::weapon_definition::WeaponDefinition
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
    Propulsion {
        max_thrust: f32,
        efficiency: f32,
    },
    Weapon(WeaponDefinition),
    Shield {
        capacity: f32,
        recharge_rate: f32,
    },
    Armor {
        hitpoints: f32,
    },
    Support {
        // flexible / misc systems
    },
}

impl Default for ModuleData {
    fn default() -> Self {
        ModuleData::Support {
        }
    }
}

#[derive(Asset, TypePath, Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ModuleDefinition {
    pub name: String,
    pub kind: ModuleData,
    pub size: ModuleSize,
}

#[derive(Asset, TypePath, Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PropulsionDefinition {
    pub max_thrust: f32,
    pub efficiency: f32,
}

use bevy_asset_loader::asset_collection::AssetCollection;

// TODO: this is bevy_asset_loader specific, keep here or move to assets.rs?
#[derive(AssetCollection, Resource)]
pub struct ModuleDefinitions {
    #[asset(path = "data/modules", collection(typed))]
    folder: Vec<Handle<ModuleDefinition>>,
}