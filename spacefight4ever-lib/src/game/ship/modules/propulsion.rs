use bevy::prelude::*;

use crate::game::ship::definitions::ship_definition::{
    ShipModel, ShipDefinition, ShipDefinitionIndex,
};
use super::stats::Stat;

/// propulsion is acutally thrust (N)
#[derive(Component, Default)]
pub struct PropulsionStat {
    current: f32,
    pub min: f32,
    pub max: f32,
}

impl Stat for PropulsionStat {
    fn set(&mut self, value: f32) {
        self.current = value.clamp(self.min, self.max);
    }

    fn get(&self) -> f32 {
        self.current
    }

    fn normalize(&self) -> f32 {
        (self.current - self.min) / (self.max - self.min)
    }

    fn min(&self) -> f32 {
        self.min
    }

    fn set_min(&mut self, value: f32) {
        self.min = value;
        self.current = self.current.clamp(self.min, self.max);
    }

    fn max(&self) -> f32 {
        self.max
    }

    fn set_max(&mut self, value: f32) {
        self.max = value;
        self.current = self.current.clamp(self.min, self.max);
    }

}

impl PropulsionStat {
    pub fn calculate_speed_max(
        &self,
        model: &ShipModel,
        index: &Res<ShipDefinitionIndex>,
        defs: &Res<Assets<ShipDefinition>>, 
    ) -> f32 {
        let Some(handle) = index.index.get(model) else { return 0.; };
        let Some(def) = defs.get(handle) else { return 0.; };
        return self.current / def.linear_dampening;
    }

    // TODO: this fails silently, make some noise ...
    pub fn calculate_accelartion_max(
        &self,
        model: &ShipModel,
        index: &Res<ShipDefinitionIndex>,
        defs: &Res<Assets<ShipDefinition>>, 
    ) -> f32 {
        let Some(handle) = index.index.get(model) else { return 0.; };
        let Some(def) = defs.get(handle) else { return 0.; };
        return self.current / def.mass;
    }
}

// speed is a 'virtual' stat that needs to be computed
// speed (m/s) = thrust (N) / mass (kg)
// i can either compute that whenever i need it, or i can compute it once when the propulsion
// module is changed (below)
// if i compute it once, i need to store it somewhere

// pub fn compute_ship_capability(
//     mut ships: Query<(&ShipModel, &Children, &mut PropulsionStat,)>,
//     modules: Query<&PropulsionModule>,
// ) {
//     for (_model, children, mut cap) in &mut ships {
//         let mut thrust_max = 0.0;
        
//         for child in children {
//             if let Ok(m) = modules.get(*child) {
//                 thrust_max += m.max_thrust * m.efficiency;
//                 //speed_max += m.max_thrust * m.efficiency / mass;
//             }
//         }

//         cap.max = thrust_max;
//     }
// }

