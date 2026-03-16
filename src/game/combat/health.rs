
use bevy::prelude::*;

use crate::game::combat::health_basetypes::*;
use crate::game::combat::ships::*;

#[derive(Component)]
pub struct ShipHealth {
    pub values: Layered<i32>,
    pub values_max: Layered<i32>
}

pub type DamageEfficiency = Layered<HealthPercents>;

// #[derive(Debug, Default)]
// pub struct HealthLayerPercents {
//     pub values: [HealthPercents; 3],
// }

// impl Index<HealthLayerType> for HealthLayerPercents {
//     type Output = HealthPercents;

//     fn index(&self, t: HealthLayerType) -> &Self::Output {
//         &self.values[t.index()]
//     }
// }

// impl IndexMut<HealthLayerType> for HealthLayerPercents {
//     fn index_mut(&mut self, t: HealthLayerType) -> &mut Self::Output {
//         &mut self.values[t.index()]
//     }
// }


// messages start here
//
//

#[derive(Message, Debug)]
pub struct HealthDamageReceived {
    pub entity: Entity,
    pub damage: i32,
    pub damage_profile: HealthPercents,
    pub damage_shield_effect: HealthPercents,
    pub damage_armor_effect: HealthPercents,
    pub damage_hull_effect: HealthPercents,
}

impl Default for HealthDamageReceived {
    fn default() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            damage: 0,
            damage_profile: HealthPercents { ..default() },
            damage_shield_effect: HealthPercents { ..default() },
            damage_armor_effect: HealthPercents { ..default() },
            damage_hull_effect: HealthPercents { ..default() },
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


fn apply_damage_vector(
    mut damage: HealthPercents,
    ship_health: &mut ShipHealth,
    layer_resistence: &ShipResistances,
) {
    for layer in HealthLayerType::ALL {
        // Calculate effective damage per type
        let mut applied = HealthPercents::default();
        let mut total = 0.0;
        for dmg_type in HealthChangeType::ALL_DAMAGE_TYPES {
            applied[dmg_type] = damage[dmg_type] * (1.0 - layer_resistence[layer][dmg_type]);
            total += applied[dmg_type];
        }

        let absorbed = total.min(ship_health.values[layer] as f32);
        ship_health.values[layer] -= absorbed as i32;

        // Scale applied damage to remaining fraction
        let fraction_remaining = if total > 0.0 { 1.0 - absorbed / total } else { 0.0 };
        for dmg_type in HealthChangeType::ALL_DAMAGE_TYPES {
            damage[dmg_type] = if layer_resistence[layer][dmg_type] < 1.0 {
                applied[dmg_type] * fraction_remaining / (1.0 - layer_resistence[layer][dmg_type])
            } else {
                0.0
            };
        }
    }
}



// Todo: Testcases
// Todo: add health max ceiling (for heal) / only necessary in apply function
// Todo: think about using i32 or i64 with fixed comma (= multiply with 1024) instead of f32 
pub fn apply_damage_system(
    mut events: MessageReader<HealthDamageReceived>,
    mut query: Query<(&mut ShipHealth, &ShipResistances)>,
) {
    for event in events.read() {
        if let Ok((mut health, resistances)) = query.get_mut(event.entity) {
            let damage = HealthPercents::split_value_by_percentages(event.damage, event.damage_profile);
            apply_damage_vector(damage, &mut health, resistances);
        }
    }
}

pub fn apply_heal_system(
    mut events: MessageReader<HealReceived>,
    mut query: Query<&mut ShipHealth>,
) {
    for event in events.read() {
        if let Ok(mut health) = query.get_mut(event.entity) {
            // health.shield = (health.shield + event.shield).clamp(0, health.shield_max);
            // health.armor = (health.armor + event.armor).clamp(0, health.armor_max);
            // health.hull = (health.hull + event.hull).clamp(0, health.hull_max);
        }
    }
}
