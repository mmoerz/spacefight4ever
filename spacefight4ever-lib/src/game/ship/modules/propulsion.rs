use bevy::prelude::*;

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
    mut ships: Query<(&Children, &mut PropulsionStat,)>,
    modules: Query<&PropulsionModule>,
) {
    for (children, mut cap) in &mut ships {
        let mut max = 0.0;
        let mut cruise = 0.0;

        for child in children {
            if let Ok(m) = modules.get(*child) {
                max += m.max_thrust * m.efficiency;
                cruise += m.max_thrust * 0.6 * m.efficiency;
            }
        }

        cap.max = max;
        cap.set(cruise);
    }
}

