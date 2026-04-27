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
    propulsion_definition::{PropulsionDefinition,PropulsionView,},
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

impl ModuleData {
    pub fn as_propulsion(&self) -> Option<PropulsionView<'_>> {
        match self {
            ModuleData::Propulsion(p) => Some(PropulsionView { inner: p }),
            _ => None,
        }
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
        &["module.def.ron"]
    }
}

use bevy_asset_loader::asset_collection::AssetCollection;

// TODO: this is bevy_asset_loader specific, keep here or move to assets.rs?
#[derive(AssetCollection, Resource)]
pub struct ModuleDefinitions {
    #[asset(path = "data/modules", collection(typed))]
    folder: Vec<Handle<ModuleDefinition>>,
}

// TODO: convert to u32 for further speed
#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct ModuleId(pub String);

/// convert str into  ModuleId
impl From<&str> for ModuleId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

/// convert str into  ModuleId
/// to avoid allocating
impl std::borrow::Borrow<str> for ModuleId {
    fn borrow(&self) -> &str {
        &self.0
    }
}

// TODO: convert to Vec<Handle<ModuleDefinition>> using predefined static ids
// TODO: for network/save systems fixed id is necessary
#[derive(Resource, Default)]
pub struct ModuleDefinitionIndex {
    index: HashMap<ModuleId, Handle<ModuleDefinition>>,
}

impl ModuleDefinitionIndex {
    pub fn get(&self, id: &ModuleId) -> Option<&Handle<ModuleDefinition>> {
        self.index.get(id)
    }

    pub fn get_str(&self, name: &str) -> Option<&Handle<ModuleDefinition>> {
        self.index.get(name)
    }
}

pub fn build_module_definition_index_once_system(
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

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use super::*;
    use bevy::ecs::system::RunSystemOnce;

    #[test]
    fn indexes_single_module() {
        let mut world = World::new();

        // Insert Assets resource
        world.init_resource::<Assets<ModuleDefinition>>();

        // Create a handle + insert asset
        let handle = {
            let mut assets = world.resource_mut::<Assets<ModuleDefinition>>();

            let handle = assets.add(ModuleDefinition {
                name: "laser".to_string(),
                ..Default::default()
            });

            handle
        };

        // Insert ModuleDefinitions resource
        world.insert_resource(ModuleDefinitions {
            folder: vec![handle.clone()],
        });

        world.insert_resource(ModuleDefinitionIndex::default());

        // Run system
        world.run_system_once(build_module_definition_index_once_system);

        // Check result
        let index = world.resource::<ModuleDefinitionIndex>();
        assert!(index.get_str("laser").is_some());
    }

    #[test]
    fn lookup_returns_correct_handle() {
        let mut world = World::new();

        // Insert Assets resource
        let mut index = ModuleDefinitionIndex::default();
        world.init_resource::<Assets<ModuleDefinition>>();

        let mut assets = world.resource_mut::<Assets<ModuleDefinition>>();

        let handle = assets.add(ModuleDefinition {
            name: "engine".to_string(),
            ..Default::default()
        });

        index.index.insert(ModuleId("engine".to_string()), handle.clone());

        let result = index.get_str("engine").unwrap();

        assert_eq!(result, &handle);
    }

    #[test]
    fn lookup_missing_returns_none() {
        let index = ModuleDefinitionIndex::default();

        assert!(index.get_str("does_not_exist").is_none());
    }

    #[test]
    fn duplicate_names_overwrite() {
        let mut world = World::new();

        world.init_resource::<Assets<ModuleDefinition>>();

        let (h1, h2) = {
            let mut assets = world.resource_mut::<Assets<ModuleDefinition>>();

            let h1 = assets.add(ModuleDefinition {
                name: "dup".into(),
                ..Default::default()
            });

            let h2 = assets.add(ModuleDefinition {
                name: "dup".into(),
                ..Default::default()
            });

            (h1, h2)
        };

        world.insert_resource(ModuleDefinitions {
            folder: vec![h1.clone(), h2.clone()],
        });

        world.insert_resource(ModuleDefinitionIndex::default());

        world.run_system_once(build_module_definition_index_once_system);

        let index = world.resource::<ModuleDefinitionIndex>();
        let result = index.get_str("dup").unwrap();

        assert_eq!(result, &h2);
    }

    #[test]
    fn skips_unloaded_assets() {
        let mut world = World::new();

        world.init_resource::<Assets<ModuleDefinition>>();

        // Create a handle WITHOUT adding asset
        let handle: Handle<ModuleDefinition> = Handle::default();

        world.insert_resource(ModuleDefinitions {
            folder: vec![handle],
        });

        world.insert_resource(ModuleDefinitionIndex::default());

        world.run_system_once(build_module_definition_index_once_system);

        let index = world.resource::<ModuleDefinitionIndex>();
        assert!(index.index.is_empty());
    }

    #[test]
    fn running_twice_does_not_duplicate_entries() {

        let mut world = World::new();

        world.init_resource::<Assets<ModuleDefinition>>();
        world.insert_resource(ModuleDefinitionIndex::default());

        let handle = {
            let mut assets = world.resource_mut::<Assets<ModuleDefinition>>();

            assets.add(ModuleDefinition {
                name: "laser".to_string(),
                ..Default::default()
            })
        };

        world.insert_resource(ModuleDefinitions {
            folder: vec![handle.clone()],
        });

        // register system once
        let system = world.register_system(build_module_definition_index_once_system);

        // run twice
        world.run_system(system);
        world.run_system(system);

        let index = world.resource::<ModuleDefinitionIndex>();

        assert_eq!(index.index.len(), 1);
    }

    #[test]
    fn as_propulsion_returns_some_for_propulsion() {
        let def = ModuleData::Propulsion(PropulsionDefinition::default());

        assert!(def.as_propulsion().is_some());
    }

    #[test]
    fn as_propulsion_returns_none_for_other_types() {
        let def = ModuleData::Support(SupportDefinition::Scan { strength: 1.0 });

        assert!(def.as_propulsion().is_none());
    }

    #[test]
    fn ron_roundtrip() {
        let original = ModuleDefinition {
            name: "shield".into(),
            size: ModuleSize::Medium,
            kind: ModuleData::Support(SupportDefinition::Scan { strength: 2.5 }),
        };

        let ron = ron::ser::to_string(&original).unwrap();
        let decoded: ModuleDefinition = ron::de::from_str(&ron).unwrap();

        assert_eq!(original, decoded);
    }

    #[test]
    fn module_size_ordering() {
        assert!(ModuleSize::Small < ModuleSize::Large);
        assert!(ModuleSize::Micro < ModuleSize::XLarge);
    }
}