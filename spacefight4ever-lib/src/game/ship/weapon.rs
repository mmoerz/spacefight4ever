use bevy::prelude::*;

use crate::game::ship::definitions::module_definition::ModuleDefinition;
use crate::game::ship::definitions::ammunition_definition::*;

#[derive(Component, Clone)]
pub struct Weapon {
    pub handle: Handle<ModuleDefinition>,
    pub cooldown: f32
}

impl Weapon {
    pub fn new(handle: Handle<ModuleDefinition>) -> Self {
        Self {
            handle,
            cooldown: 0.0,
        }
    }
}

#[derive(Component)]
pub struct Target;

#[derive(Component, Clone, Copy)]
pub struct Ammunition {
    pub ammo_id: AmmunitionDefinitionId,
    pub count: i32,
}

#[derive(Component, Clone, Copy)]
pub struct Missile {
    pub origin: Vec3,
    pub target: Entity,
    pub base_damage: f32, // base dmg from weapon, additional dmg from ammunition
    pub ammo_id: AmmunitionDefinitionId,
    pub fuel: f32,
}


#[derive(Component, Clone, Copy)]
pub struct Projectile {
    pub origin: Vec3,
    pub target: Entity,
    pub base_damage: f32, // base dmg from weapon, additional dmg from ammunition
    pub ammo_id: AmmunitionDefinitionId,
}

