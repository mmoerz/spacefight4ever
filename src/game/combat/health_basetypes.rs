use bevy::prelude::*;
use std::ops::{Index, IndexMut};

// Todo: figure out if Type postfix is really necessary
#[derive(Clone, Copy, Debug)]
pub enum HealthChangeType {
    Kinetic,
    Thermal,
    Explosive,
    Electromagnetic
}

impl HealthChangeType {
    pub fn index(self) -> usize {
        match self {
            HealthChangeType::Kinetic => 0,
            HealthChangeType::Thermal => 1,
            HealthChangeType::Explosive => 2,
            HealthChangeType::Electromagnetic => 3,
        }
    }
    pub const ALL_DAMAGE_TYPES: [HealthChangeType; 4] = [
        HealthChangeType::Kinetic,
        HealthChangeType::Thermal,
        HealthChangeType::Explosive,
        HealthChangeType::Electromagnetic,
    ];
}

#[derive(Debug, Default, Clone, Copy)]
pub struct HealthPercents {
    pub values: [f32; 4],
}

impl Index<HealthChangeType> for HealthPercents {
    type Output = f32;

    fn index(&self, t: HealthChangeType) -> &Self::Output {
        &self.values[t.index()]
    }
}

impl IndexMut<HealthChangeType> for HealthPercents {
    fn index_mut(&mut self, t: HealthChangeType) -> &mut Self::Output {
        &mut self.values[t.index()]
    }
}

impl HealthPercents {
    pub fn split_value_by_percentages(value: i32, percentages: HealthPercents) -> HealthPercents {
        let mut res = HealthPercents { ..default() };

        for changetype in HealthChangeType::ALL_DAMAGE_TYPES {
            res[changetype] = value as f32 * percentages[changetype];
        }

        res
    }
}

#[derive(Clone, Copy, Debug)]
pub enum HealthLayerType {
    Shield,
    Armor,
    Hull
}

impl HealthLayerType {
    pub fn index(self) -> usize {
        match self {
            HealthLayerType::Shield => 0,
            HealthLayerType::Armor => 1,
            HealthLayerType::Hull => 2,
        }
    }
    pub const ALL: [HealthLayerType; 3] = [
        HealthLayerType::Shield,
        HealthLayerType::Armor,
        HealthLayerType::Hull,
    ];
}

/// Generic container for per-layer values
#[derive(Debug, Default, Clone, Copy)] 
pub struct Layered<T: Default + Copy> {
     pub values: [T; 3], 
} 

impl<T: Default + Copy> Index<HealthLayerType> for Layered<T> {
    type Output = T;
     
    fn index(&self, layer: HealthLayerType) -> &Self::Output {
        &self.values[layer.index()] 
    } 
} 

impl<T: Default + Copy> IndexMut<HealthLayerType> for Layered<T> {
    fn index_mut(&mut self, layer: HealthLayerType) -> &mut Self::Output {
        &mut self.values[layer.index()] 
    } 
}
