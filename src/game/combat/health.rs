use std::ops::RemAssign;

use bevy::prelude::*;
use crate::game::combat::ships::*;


#[derive(Component)]
pub struct ShipHealth {
    pub shield: i32,
    pub shield_max: i32,
    pub armor: i32,
    pub armor_max: i32,
    pub hull: i32,
    pub hull_max: i32,
}

#[derive(Clone, Copy)]
pub enum HealthType {
    Shield,
    Armor,
    Hull
}

#[derive(Clone, Copy, Debug)]
pub enum DamageType {
    None,
    Kinetic,
    Thermal,
    Explosiv,
    Electromagnetic,
    Allround
}

#[derive(Debug, Default)]
pub struct DamageProfile {
    kinetic: f32,
    thermal: f32,
    explosiv: f32,
    electromagnetic: f32,
}

#[derive(Debug, Default)]
pub struct DamageEffectivity {
    kinetic: f32,
    thermal: f32,
    explosiv: f32,
    electromagnetic: f32,
}

#[derive(Debug, Default)]
pub struct ProfiledAmount {
    kinetic: i32,
    thermal: i32,
    explosiv: i32,
    electromagnetic: i32,
}

#[derive(Message, Debug)]
pub struct HealthDamageReceived {
    pub entity: Entity,
    pub damage: i32,
    pub damage_profile: DamageProfile,
    pub damage_shield_effect: DamageEffectivity,
    pub damage_armor_effect: DamageEffectivity,
    pub damage_hull_effect: DamageEffectivity,
}

impl Default for HealthDamageReceived {
    fn default() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            damage: 0,
            damage_profile: DamageProfile { ..default() },
            damage_shield_effect: DamageEffectivity { ..default() },
            damage_armor_effect: DamageEffectivity { ..default() },
            damage_hull_effect: DamageEffectivity { ..default() },
        }
    }
}

#[derive(Message, Debug)]
pub struct HealReceived {
    pub entity: Entity,
    pub shield: i32,
    pub armor: i32,
    pub hull: i32,
}

impl Default for HealReceived {
    fn default() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            shield: 0,
            armor: 0,
            hull: 0,
        }
    }
}

pub fn apply_damage_system(
    mut events: MessageReader<HealthDamageReceived>,
    mut query: Query<(&mut ShipHealth, &ShipResistances)>,
) {
    for event in events.read() {
        if let Ok((mut health, resistances)) = query.get_mut(event.entity) {
            // incoming damage application that uses
            // the resistence profil of the ship to apply the damage
            let damage = event.damage;

            let mut remaining = apply_profile_to_damage(
                event.damage, 
                event.damage_profile, 
                event.damage_shield_effect
            );

            // Shield absorbs first
            let shield_profiled = split_health_according_to_profile(health.shield, event.damage_profile);
            
            let shield_absorb = remaining.min(health.shield);
            health.shield -= shield_absorb;
            remaining -= shield_absorb;

            // Armor absorbs second
            let armor_absorb = remaining.min(health.armor);
            health.armor -= armor_absorb;
            remaining -= armor_absorb;

            // Remaining damage hits hull
            health.hull -= remaining;
            if health.hull < 0 {
                health.hull = 0;
            }
        }
    }
}

pub fn apply_profile_to_damage(
    damage: i32,
    damage_profile: DamageProfile,
    effectivity: DamageEffectivity
) -> ProfiledAmount {
    ProfiledAmount { 
        kinetic: (damage as f32 * damage_profile.kinetic * effectivity.kinetic) as i32,
        thermal: (damage as f32 * damage_profile.thermal * effectivity.thermal) as i32,
        explosiv: (damage as f32 * damage_profile.explosiv * effectivity.explosiv) as i32,
        electromagnetic: (damage as f32 * damage_profile.electromagnetic * effectivity.electromagnetic) as i32,
    }
}

pub fn split_health_according_to_profile(
    health: i32,
    damage_profile: DamageProfile
) -> ProfiledAmount {
    ProfiledAmount {
        kinetic: (health as f32 * damage_profile.kinetic) as i32,
        thermal: (health as f32 * damage_profile.thermal) as i32,
        explosiv: (health as f32 * damage_profile.explosiv) as i32,
        electromagnetic: (health as f32 * damage_profile.electromagnetic) as i32,
    }
}

pub fn apply_heal_system(
    mut events: MessageReader<HealReceived>,
    mut query: Query<&mut ShipHealth>,
) {
    for event in events.read() {
        if let Ok(mut health) = query.get_mut(event.entity) {
            health.shield = (health.shield + event.shield).clamp(0, health.shield_max);
            health.armor = (health.armor + event.armor).clamp(0, health.armor_max);
            health.hull = (health.hull + event.hull).clamp(0, health.hull_max);
        }
    }
}
