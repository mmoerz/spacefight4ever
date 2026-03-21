use std::ops::{Index, IndexMut};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::ship::{definition_repository::{DefinitionRepository, NamedDefinition}, module::ModuleSize};

const PATH_WEAPON_DEFINITION: &str = "assets/data/weapons.json";

/// describes the static data of a weapon

// for storing weapon definitions that are reused between identical weapons
// what should be possible weapons
//    Missile,
    // Gattling,
    // Railgun,
    // Gauss,
    // Particle,
    // Ion,
    // Laser,
    // Plasma

#[derive(Clone, Copy, Debug)]
pub enum WeaponRangeType {
    Max,
    Optimal,
    Min,
}

impl WeaponRangeType {
    pub fn index(self) -> usize {
        match self {
            WeaponRangeType::Min => 0,
            WeaponRangeType::Optimal => 1,
            WeaponRangeType::Max => 2,
        }
    }
    pub const ALL: [WeaponRangeType; 3] = [
        WeaponRangeType::Max,
        WeaponRangeType::Optimal,
        WeaponRangeType::Min,
    ];
}

/// Generic container for per-layer values
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Deserialize, Serialize)] 
pub struct WeaponRange<T: Default + Copy> {
     pub values: [T; 3], 
} 

impl<T: Default + Copy> Index<WeaponRangeType> for WeaponRange<T> {
    type Output = T;
     
    fn index(&self, layer: WeaponRangeType) -> &Self::Output {
        &self.values[layer.index()] 
    } 
}

impl<T: Default + Copy> IndexMut<WeaponRangeType> for WeaponRange<T> {
    fn index_mut(&mut self, layer: WeaponRangeType) -> &mut Self::Output {
        &mut self.values[layer.index()] 
    }
}

impl<T: Default + Copy> WeaponRange<T> {
    pub fn new(min: T, optimal: T, max: T) -> Self {
        Self {
            values: [min, optimal, max],
        }
    }
    pub fn min(&self) -> T { self[WeaponRangeType::Min] }
    pub fn optimal(&self) -> T { self[WeaponRangeType::Optimal] }
    pub fn max(&self) -> T { self[WeaponRangeType::Max] }
}

impl<T: Default + Copy + PartialOrd> WeaponRange<T>{
    pub fn is_valid(&self) -> bool {
        self.values[0] < self.values[1] && self.values[1] < self.values[2]
    }
}

/// different ways of weapon behaviour that can be handled
/// 
/// * Beam is instant weapon damage
/// * Missile creates a missile that is then simulated
/// * Projectile creates a projectile that is then simulated
#[derive(Component, Default, Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum WeaponBehavior{
    #[default]
    Beam, /// instant damage
    Missile,
    Projectile
}

/// describes the static data of a weapon
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct WeaponDefinition {
    pub name: String,
    pub size: ModuleSize,
    pub behavior: WeaponBehavior,
    pub range: WeaponRange<f32>,
    pub max_angle: Option<f32>,
    pub fire_rate: f32,
    pub damage: f32,
    pub ammo_max: i32,
}

impl NamedDefinition for WeaponDefinition {
    fn name(&self) -> &str {
        &self.name
    }
}

/// just an alias to usize for weapon ids
/// 
/// usize is safer (no overflow), but it uses up i64 on 64bit systems
/// u32 uses less memory, better for ECS
pub type WeaponDefinitionId = u32;

/// Manager for weapon definitions.
/// 
/// stores weapon definitions in an immuteable Vec
/// 
/// weapon defintions are retrieveable by id and name
/// 
/// name lookups use a hashmap for fast retrieval
#[derive(Resource)]
pub struct WeaponDefinitionRepository(pub DefinitionRepository<WeaponDefinition>);

impl WeaponDefinitionRepository {
    /// Load weapon definitions from a JSON file.
    /// 
    /// ## Arguments
    /// 
    /// * `path` - The path to the JSON file containing the weapon definitions.
    pub fn load_from_file(path: &str) -> Self {
        let data = std::fs::read_to_string(path).expect("Failed to read weapons file");
        let defs: Vec<WeaponDefinition> =
            serde_json::from_str(&data).expect("Failed to parse weapon definitions");

        Self(DefinitionRepository::from_vec(defs))
    }

    pub fn from_vec(defs: Vec<WeaponDefinition>) -> Self {
        Self(DefinitionRepository::from_vec(defs))
    }

    pub fn get_by_id(&self, id: WeaponDefinitionId) -> &WeaponDefinition {
        self.0.get_by_id(id as usize)
    }
    pub fn get_by_name(&self, name: &str) -> Option<&WeaponDefinition> {
        self.0.get_by_name(name)
    }
    pub fn has_id(&self, id: WeaponDefinitionId) -> bool {
        self.0.has_id(id as usize)
    }
    pub fn has_name(&self, name: &str) -> bool {
        self.0.has_name(name)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

pub fn setup_weapon_repo(mut commands: Commands) {
    let repo = WeaponDefinitionRepository::load_from_file(PATH_WEAPON_DEFINITION);
    commands.insert_resource(repo);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_weapon_defs() -> Vec<WeaponDefinition> {
        vec![
            WeaponDefinition {
                name: "Laser".to_string(),
                size: ModuleSize::Small,
                behavior: WeaponBehavior::Beam,
                range: WeaponRange::new(10.0, 50.0, 100.0),
                max_angle: Some(5.0),
                fire_rate: 1.0,
                damage: 100.0,
                ammo_max: 50,
            },
            WeaponDefinition {
                size: ModuleSize::Medium,
                name: "Missile".to_string(),
                behavior: WeaponBehavior::Missile,
                range: WeaponRange::new(20.0, 100.0, 200.0),
                max_angle: Some(2.5),
                fire_rate: 0.5,
                damage: 250.0,
                ammo_max: 10,
            },
        ]
    }

    fn repo_from_defs(defs: Vec<WeaponDefinition>) -> WeaponDefinitionRepository {
        WeaponDefinitionRepository(DefinitionRepository::from_vec(defs))
    }

    #[test]
    fn test_get_by_id_returns_correct_definition() {
        let repo = repo_from_defs(sample_weapon_defs());

        let laser = repo.get_by_id(0);
        assert_eq!(laser.name, "Laser");
        assert_eq!(laser.behavior, WeaponBehavior::Beam);

        let missile = repo.get_by_id(1);
        assert_eq!(missile.name, "Missile");
        assert_eq!(missile.behavior, WeaponBehavior::Missile);
    }

    #[test]
    fn test_get_by_name_returns_correct_definition() {
        let repo = repo_from_defs(sample_weapon_defs());

        let laser = repo.get_by_name("Laser").unwrap();
        assert_eq!(laser.range.min(), 10.0);
        assert_eq!(laser.range.optimal(), 50.0);
        assert_eq!(laser.range.max(), 100.0);

        let missile = repo.get_by_name("Missile").unwrap();
        assert_eq!(missile.damage, 250.0);
        assert_eq!(missile.ammo_max, 10);
    }

    #[test]
    fn test_get_by_name_returns_none_for_invalid_name() {
        let repo = repo_from_defs(sample_weapon_defs());

        let none = repo.get_by_name("UnknownWeapon");
        assert!(none.is_none());
    }

    #[test]
    fn test_consistency_between_id_and_name_lookup() {
        let repo = repo_from_defs(sample_weapon_defs());

        for id in 0..repo.len() {
            let def_by_id = repo.get_by_id(id as WeaponDefinitionId);
            let def_by_name = repo.get_by_name(&def_by_id.name).unwrap();
            assert_eq!(def_by_id, def_by_name);
        }
    }

    #[test]
    #[should_panic]
    fn test_get_by_id_panics_on_out_of_bounds() {
        let repo = repo_from_defs(sample_weapon_defs());
        let _ = repo.get_by_id(10); // Should panic
    }

    #[test]
    fn test_has_id() {
        let repo = repo_from_defs(sample_weapon_defs());

        // Valid IDs
        assert!(repo.has_id(0));
        assert!(repo.has_id(1));

        // Invalid IDs
        assert!(!repo.has_id(2));
        assert!(!repo.has_id(999));
    }

    #[test]
    fn test_has_name() {
        let repo = repo_from_defs(sample_weapon_defs());

        // Existing names
        assert!(repo.has_name("Laser"));
        assert!(repo.has_name("Missile"));

        // Non-existing names
        assert!(!repo.has_name("Railgun"));
        assert!(!repo.has_name(""));
    }

    // ensure that both fields have consistent data
    #[test]
    fn test_consistency_with_getters() {
        let repo = repo_from_defs(sample_weapon_defs());

        for id in 0..repo.len() {
            assert!(repo.has_id(id as WeaponDefinitionId));

            let def = repo.get_by_id(id as WeaponDefinitionId);
            assert!(repo.has_name(&def.name));

            let by_name = repo.get_by_name(&def.name).unwrap();
            assert_eq!(by_name, def);
        }
    }
}