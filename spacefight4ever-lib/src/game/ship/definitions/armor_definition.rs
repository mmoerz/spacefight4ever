use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ArmorDefinition {
    pub hitpoints: f32,
}