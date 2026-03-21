use bevy::prelude::*;
use std::ops::{Index, IndexMut};

/// contains the diffent possible damage types
/// 
/// **Usage notes:** - You can iterate over `HealthChangeType::ALL` when applying or calculating damage for each type
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
    pub const ALL: [HealthChangeType; 4] = [
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

        for changetype in HealthChangeType::ALL {
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
pub struct LayeredHealth<T: Default + Copy> {
     pub values: [T; 3], 
} 

impl<T: Default + Copy> Index<HealthLayerType> for LayeredHealth<T> {
    type Output = T;
     
    fn index(&self, layer: HealthLayerType) -> &Self::Output {
        &self.values[layer.index()] 
    } 
} 

impl<T: Default + Copy> IndexMut<HealthLayerType> for LayeredHealth<T> {
    fn index_mut(&mut self, layer: HealthLayerType) -> &mut Self::Output {
        &mut self.values[layer.index()] 
    } 
}

#[derive(Message, Debug, Clone, Copy)]
pub struct HealthDamageAbsorbed
{
    pub entity: Entity,
    pub damage: HealthPercents,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct HealthHealAbsorbed
{
    pub entity: Entity,
    pub healing: LayeredHealth<i32>,
}

pub type DamageEfficiency = LayeredHealth<HealthPercents>;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_value_by_percentages() {
        let total_value = 100;
        let mut percentages = HealthPercents::default();
        percentages[HealthChangeType::Kinetic] = 0.25;
        percentages[HealthChangeType::Thermal] = 0.25;
        percentages[HealthChangeType::Explosive] = 0.25;
        percentages[HealthChangeType::Electromagnetic] = 0.25;

        let result = HealthPercents::split_value_by_percentages(total_value, percentages);

        assert_eq!(result[HealthChangeType::Kinetic], 25.0);
        assert_eq!(result[HealthChangeType::Thermal], 25.0);
        assert_eq!(result[HealthChangeType::Explosive], 25.0);
        assert_eq!(result[HealthChangeType::Electromagnetic], 25.0);
    }
}