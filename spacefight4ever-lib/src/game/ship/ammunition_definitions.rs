use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::{combat::health_basetypes::{DamageEfficiency, HealthPercents}};
use crate::game::ship::definition_repository::{NamedDefinition, DefinitionRepository};

pub const PATH_AMMUNITION_DEFINITION: &str = "assets/weapons.json";

/// describes the static data of a weapon
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AmmunitionDefinition {
    pub name: String,
    pub range_modifier: f32,
    pub damage_profile: HealthPercents,
    pub damage_efficiency: DamageEfficiency,
    pub additional_damage: f32,
    pub missile_fuel_max: Option<i32>, // fuel only necessary for missiles
}

impl NamedDefinition for AmmunitionDefinition {
    fn name(&self) -> &str {
        &self.name
    }
}

/// just an alias to usize for ammunition ids
///
/// usize is safer (no overflow), but it uses up i64 on 64bit systems
/// u32 uses less memory, better for ECS
pub type AmmunitionId = usize;

/// Manager for ammunition definitions.
///
/// stores ammunition definitions in an immuteable Vec
///
/// ammunition defintions are retrieveable by id and name
///
/// name lookups use a hashmap for fast retrieval
#[derive(Resource, Default)]
pub struct AmmunitionDefinitionRepository(pub DefinitionRepository<AmmunitionDefinition>);

impl AmmunitionDefinitionRepository {
    /// Load weapon definitions from a JSON file.
    /// 
    /// ## Arguments
    /// 
    /// * `path` - The path to the JSON file containing the weapon definitions.
    pub fn load_from_file(path: &str) -> Self {
        let data = std::fs::read_to_string(path).expect("Failed to read weapons file");
        let defs: Vec<AmmunitionDefinition> =
            serde_json::from_str(&data).expect("Failed to parse weapon definitions");

        Self(DefinitionRepository::from_vec(defs))
    }
    pub fn get_by_id(&self, id: AmmunitionId) -> &AmmunitionDefinition {
        self.0.get_by_id(id)
    }
    pub fn get_by_name(&self, name: &str) -> Option<&AmmunitionDefinition> {
        self.0.get_by_name(name)
    }
    pub fn has_id(&self, id: AmmunitionId) -> bool {
        self.0.has_id(id)
    }
    pub fn has_name(&self, name: &str) -> bool {
        self.0.has_name(name)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

/// setup Manager for ammunition definitions to be injected into systems
pub fn setup_ammunition_repo(mut commands: Commands) {
    let repo = AmmunitionDefinitionRepository::load_from_file(PATH_AMMUNITION_DEFINITION);
    commands.insert_resource(repo);
}