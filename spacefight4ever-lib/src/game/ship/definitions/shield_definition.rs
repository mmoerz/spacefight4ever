use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ShieldDefinition {
    pub capacity: f32,
    pub recharge_rate: f32,
}