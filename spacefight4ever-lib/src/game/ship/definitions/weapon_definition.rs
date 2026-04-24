use std::ops::{Index, IndexMut};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// describes the static data of a weapon

// for storing weapon definitions that are reused between identical weapons
// what should be possible weapons
//    Missile,
    // Gattling,
    // Railgun,
    // Gauss,
    // Particle,
    // Ion,
    // Laser,
    // Plasma

#[derive(Clone, Copy, Debug)]
pub enum WeaponRangeType {
    Max,
    Optimal,
    Min,
}

impl WeaponRangeType {
    pub fn index(self) -> usize {
        match self {
            WeaponRangeType::Min => 0,
            WeaponRangeType::Optimal => 1,
            WeaponRangeType::Max => 2,
        }
    }
    pub const ALL: [WeaponRangeType; 3] = [
        WeaponRangeType::Max,
        WeaponRangeType::Optimal,
        WeaponRangeType::Min,
    ];
}

/// Generic container for per-layer values
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Deserialize, Serialize)] 
pub struct WeaponRange<T: Default + Copy> {
     pub values: [T; 3], 
} 

impl<T: Default + Copy> Index<WeaponRangeType> for WeaponRange<T> {
    type Output = T;
     
    fn index(&self, layer: WeaponRangeType) -> &Self::Output {
        &self.values[layer.index()] 
    } 
}

impl<T: Default + Copy> IndexMut<WeaponRangeType> for WeaponRange<T> {
    fn index_mut(&mut self, layer: WeaponRangeType) -> &mut Self::Output {
        &mut self.values[layer.index()] 
    }
}

impl<T: Default + Copy> WeaponRange<T> {
    pub fn new(min: T, optimal: T, max: T) -> Self {
        Self {
            values: [min, optimal, max],
        }
    }
    pub fn min(&self) -> T { self[WeaponRangeType::Min] }
    pub fn optimal(&self) -> T { self[WeaponRangeType::Optimal] }
    pub fn max(&self) -> T { self[WeaponRangeType::Max] }
}

impl<T: Default + Copy + PartialOrd> WeaponRange<T>{
    pub fn is_valid(&self) -> bool {
        self.values[0] < self.values[1] && self.values[1] < self.values[2]
    }
}

/// different ways of weapon behaviour that can be handled
/// 
/// * Beam is instant weapon damage
/// * Missile creates a missile that is then simulated
/// * Projectile creates a projectile that is then simulated
#[derive(Component, Default, Debug, Copy, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum WeaponBehavior{
    #[default]
    Beam, /// instant damage
    Missile,
    Projectile
}

/// describes the static data of a weapon
#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct WeaponDefinition {
    pub behavior: WeaponBehavior,
    pub range: WeaponRange<f32>,
    pub max_angle: Option<f32>,
    pub fire_rate: f32,
    pub damage: f32,
    pub ammo_max: i32,
}
