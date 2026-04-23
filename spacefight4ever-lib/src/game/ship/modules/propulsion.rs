use bevy::prelude::*;

use crate::game::ship::definitions::ship_definition::{
    ShipModel, ShipDefinition, ShipDefinitionIndex,
};
use super::stats::Stat;

#[derive(Component, Clone)]
pub struct PropulsionModule {
    pub max_thrust: f32,
    pub efficiency: f32,
}

// #[derive(Component, Default)]
// pub struct PropulsionCapability {
//     pub max_speed_estimate: f32,
//     pub cruise_speed_estimate: f32,
// }

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

    fn max(&self) -> f32 {
        self.max
    }
}

pub fn compute_ship_capability(
    mut ships: Query<(&ShipModel, &Children, &mut PropulsionStat,)>,
    modules: Query<&PropulsionModule>,
    index: Res<ShipDefinitionIndex>,
    defs: Res<Assets<ShipDefinition>>, 
) {
    for (model, children, mut cap) in &mut ships {
        let mut thrust_max = 0.0;
        let mut cruise_max = 0.0;
        let Some(handle) = index.index.get(model) else { continue; };
        let Some(def) = defs.get(handle) else { continue; };
        let mass = def.mass;

        for child in children {
            if let Ok(m) = modules.get(*child) {
                thrust_max += m.max_thrust * m.efficiency;
                cruise_max += m.max_thrust * m.efficiency / mass;
            }
        }

        cap.max = thrust_max;
        cap.set(cruise_max);
    }
}

