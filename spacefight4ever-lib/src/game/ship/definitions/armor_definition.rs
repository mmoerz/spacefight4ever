use serde::{Deserialize, Serialize};

/// defines an armor
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ArmorDefinition {
    pub hitpoints: f32,
}