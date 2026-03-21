use std::ops::{Index, IndexMut};
use bevy::{prelude::*, render::extract_component::ExtractComponent};

use crate::game::combat::health_basetypes::{DamageEfficiency, HealthPercents};

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
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)] 
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

// what should be possible weapons
//    Missile,
    // Gattling,
    // Railgun,
    // Gauss,
    // Particle,
    // Ion,
    // Laser,
    // Plasma

// #[derive(Component)]
// pub enum WeaponType {
//     Missile,
//     Gattling,
//     Railgun,
//     Gauss,
//     Particle,
//     Ion,
//     Laser,
//     Plasma,
// }

#[derive(Component, Copy, Clone, PartialEq, Eq)]
pub enum WeaponBehavior{
    Beam,
    Missile,
    Projectile
}

#[derive(Component, Clone, Copy)]
pub struct WeaponStats {
    pub range: WeaponRange<f32>,
    pub max_angle: Option<f32>,
    pub fire_rate: f32,
    pub damage: f32,
    pub ammo_max: i32,
}

#[derive(Component, Clone, Copy)]
pub struct Weapon {
    pub weapon_definition: Entity,
    pub ammo : Entity,
    pub cooldown: f32
}

impl Weapon {
    pub fn new(weapon_definition: Entity, ammo: Entity) -> Self {
        Self {
            weapon_definition,
            ammo,
            cooldown: 0.0,
        }
    }
}

#[derive(Component)]
pub struct Target;

pub struct AmmunitionType {
}

#[derive(Component, Clone, Copy)]
pub struct Ammunition {
    pub range_modifier: f32,
    pub damage_profile: HealthPercents,
    pub damage_efficiency: DamageEfficiency,
    pub additional_damage: f32,
    pub count: i32,
}