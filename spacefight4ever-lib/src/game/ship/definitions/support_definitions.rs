use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum SupportDefinition {
    Scan {
        strength: f32,
    }
}

impl Default for SupportDefinition {
    fn default() -> Self {
        SupportDefinition::Scan { strength: 0.0 }
    }
}