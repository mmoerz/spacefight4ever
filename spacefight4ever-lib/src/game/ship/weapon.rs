use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::ship::weapon_definition::*;
use crate::game::ship::ammunition_definitions::*;

#[derive(Component, Clone, Copy)]
pub struct Weapon {
    pub weapon_id: WeaponId,
    pub ammo : Entity,
    pub cooldown: f32
}

impl Weapon {
    pub fn new(weapon_id: WeaponId, ammo: Entity) -> Self {
        Self {
            weapon_id,
            ammo,
            cooldown: 0.0,
        }
    }
}

#[derive(Component)]
pub struct Target;

#[derive(Component, Clone, Copy)]
pub struct Ammunition {
    pub ammo_id: AmmunitionId,
    pub count: i32,
}

#[derive(Component, Clone, Copy)]
pub struct Missile {
    pub fuel: i32,
}