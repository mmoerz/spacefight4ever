use bevy::prelude::*;

use std::collections::HashMap;

/// for 'forcing' a name on a struct
pub trait NamedDefinition {
    fn name(&self) -> &str;
}

/// Manager for definitions aka. static data that is shared between multiple entities.
/// 
/// stores definitions in an immuteable Vec
/// 
/// defintions can be retrieved by id and name
/// 
/// name lookups use a hashmap for fast retrieval
#[derive(Resource, Default)]
pub struct DefinitionRepository<T> {
    items_by_id: Vec<T>,
    items_by_name: HashMap<String, usize>, // usize index into items_by_id
}

impl<T> DefinitionRepository<T>
where
    T: NamedDefinition,
{
    /// Create a new repository from a vector of definitions and a function to extract the name
    pub fn from_vec(defs: Vec<T>) -> Self
    {
        let items_by_name = defs
            .iter()
            .enumerate()
            .map(|(i, item)| (item.name().to_string(), i as usize))
            .collect();

        Self {
            items_by_id: defs,
            items_by_name,
        }
    }

    /// Get an item definition by its ID.
    /// 
    /// ## Arguments
    /// 
    /// * `id` - The ID of the item definition to retrieve.
    /// 
    /// ## Returns
    /// 
    /// A reference to the item definition with the specified ID.
    /// 
    /// ## Panics
    /// 
    /// If the ID is out of range.    
    pub fn get_by_id(&self, id: usize) -> &T {
        &self.items_by_id[id as usize]
    }

    /// Get a item definition by its name.
    /// 
    /// ## Arguments
    /// 
    /// * `name` - The name of the item definition to retrieve.
    /// 
    /// ## Returns
    /// 
    /// An optional reference to the item definition with the specified name.
    /// 
    /// ## Panics
    /// 
    /// If the name is not found.
    pub fn get_by_name(&self, name: &str) -> Option<&T> {
        self.items_by_name.get(name).map(|&i| &self.items_by_id[i as usize])
    }

    /// Returns `true` if a item with the given ID exists in the repository.
    ///
    /// # Arguments
    ///
    /// * `id` - The item ID to check for existence.
    ///
    pub fn has_id(&self, id: usize) -> bool {
        id < self.items_by_id.len()
    }

    /// Returns `true` if a item with the given name exists in the repository.
    ///
    /// # Arguments
    ///
    /// * `name` - The item name to check for existence.
    ///
    pub fn has_name(&self, name: &str) -> bool {
        self.items_by_name.contains_key(name)
    }

    /// Returns total number of items in the repository.
    ///
    pub fn len(&self) -> usize {
        self.items_by_id.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct TestDef {
        name: String,
        value: i32,
    }

    impl NamedDefinition for TestDef {
    fn name(&self) -> &str {
        &self.name
    }
}

    fn make_repo() -> DefinitionRepository<TestDef> {
        let defs = vec![
            TestDef { name: "Laser".into(), value: 10 },
            TestDef { name: "Missile".into(), value: 20 },
            TestDef { name: "Railgun".into(), value: 30 },
        ];

        DefinitionRepository::from_vec(defs)
    }

    #[test]
    fn from_vec_builds_repository() {
        let repo = make_repo();

        assert_eq!(repo.len(), 3);
    }

    #[test]
    fn get_by_id_returns_correct_item() {
        let repo = make_repo();

        let item = repo.get_by_id(1);

        assert_eq!(item.name, "Missile");
        assert_eq!(item.value, 20);
    }

    #[test]
    fn get_by_name_returns_correct_item() {
        let repo = make_repo();

        let item = repo.get_by_name("Railgun").unwrap();

        assert_eq!(item.value, 30);
    }

    #[test]
    fn get_by_name_returns_none_for_missing() {
        let repo = make_repo();

        assert!(repo.get_by_name("Unknown").is_none());
    }

    #[test]
    fn has_id_true_for_valid_ids() {
        let repo = make_repo();

        assert!(repo.has_id(0));
        assert!(repo.has_id(1));
        assert!(repo.has_id(2));
    }

    #[test]
    fn has_id_false_for_invalid_ids() {
        let repo = make_repo();

        assert!(!repo.has_id(3));
        assert!(!repo.has_id(999));
    }

    #[test]
    fn has_name_true_for_existing() {
        let repo = make_repo();

        assert!(repo.has_name("Laser"));
        assert!(repo.has_name("Missile"));
    }

    #[test]
    fn has_name_false_for_missing() {
        let repo = make_repo();

        assert!(!repo.has_name("Plasma"));
    }

    #[test]
    fn len_returns_number_of_items() {
        let repo = make_repo();

        assert_eq!(repo.len(), 3);
    }

    #[test]
    fn id_and_name_lookup_are_consistent() {
        let repo = make_repo();

        for id in 0..repo.len() {
            let by_id = repo.get_by_id(id);
            let by_name = repo.get_by_name(&by_id.name).unwrap();

            assert_eq!(by_id, by_name);
        }
    }
}