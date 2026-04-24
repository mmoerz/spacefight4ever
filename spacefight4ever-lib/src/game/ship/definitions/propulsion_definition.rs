use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PropulsionDefinition {
    pub max_thrust: f32,
    pub efficiency: f32,
}

pub struct PropulsionView<'a> {
    pub inner: &'a PropulsionDefinition,
}