// Todo: think about using i32 or i64 with fixed comma (= multiply with 1024) instead of f32 
// Todo: when implementing networking i32 might come in handy (f32 is tricky to transport)

use bevy::camera::visibility::Layer;
use bevy::prelude::*;

use crate::game::combat::health_basetypes::*;
use crate::game::combat::ships::*;

#[derive(Component)]
pub struct ShipHealth {
    pub values: LayeredHealth<i32>,
    pub values_max: LayeredHealth<i32>
}

pub type DamageEfficiency = LayeredHealth<HealthPercents>;

// messages start here
//
//

#[derive(Message, Debug)]
pub struct HealthDamageReceived {
    pub entity: Entity,
    pub damage: i32,
    pub damage_profile: HealthPercents,
    pub damage_efficiency: DamageEfficiency,
}

impl Default for HealthDamageReceived {
    fn default() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            damage: 0,
            damage_profile: HealthPercents { ..default() },
            damage_efficiency: DamageEfficiency { ..default() },
        }
    }
}

pub type HealthHealing  = LayeredHealth<i32>;

/// for sending heal messages
#[derive(Message, Debug)]
pub struct HealReceived {
    pub entity: Entity,
    pub healing: HealthHealing,
}

impl Default for HealReceived {
    fn default() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            healing: HealthHealing {
                values: [0, 0, 0],
            },
        }
    }
}

/// Applies a damage vector to a ship's layered health, taking into account
/// resistances and damage efficiencies per layer and per damage type.
///
/// This function iterates over each health layer (shield, armor, hull) and:
/// 1. Computes the effective damage per damage type using:
///      applied_damage = incoming_damage * damage_efficiency * (1 - layer_resistance)
/// 2. Applies the total effective damage to the layer, capping at the current health.
/// 3. Computes any remaining damage that could overflow to the next layer,
///    scaling it back into a damage vector with resistances and efficiencies reversed.
///
/// # Damage Flow Diagram
/// ```text
/// Incoming Damage ──▶ Shield ──▶ Armor ──▶ Hull
///       │                │          │
///       │      (absorbs up to current health)
///       │                │          │
///       └─> Remaining ──> Remaining ──> Remaining applied to next layer
/// ```
/// Each arrow represents the propagation of remaining damage to the next layer.
///
/// # Parameters
/// - `damage`: The incoming damage as `HealthPercents`, representing damage amounts per type.
///             This vector will be **mutated** to hold remaining damage after each layer.
/// - `damage_efficiency`: The `DamageEfficiency` matrix defining how effective each damage type
///                        is against each health layer.
/// - `ship_health`: Mutable reference to the `ShipHealth` struct, storing current and max health
///                  for each layer. Health is reduced based on the applied damage.
/// - `layer_resistence`: The `ShipResistances` struct, storing resistances per layer and per
///                       damage type. Resistances reduce incoming damage.
///
/// # Behavior
/// - Damage is applied in the order: shield → armor → hull.
/// - Damage is capped by the current health of the layer (`ship_health.values[layer]`).
/// - Any remaining damage after one layer is scaled and propagated to the next layer.
/// - The `damage` vector is updated to represent the portion of damage still to be applied.
///
/// # Notes
/// - Health values are `i32`, while damage, resistances, and efficiencies are `f32`.
/// - The function assumes `damage_efficiency` and `layer_resistence` values are in `[0.0, 1.0]`.
/// - Fractional damage is truncated to integer health when applied.
///
fn apply_damage_vector(
    mut damage: HealthPercents,
    damage_efficiency: DamageEfficiency,
    ship_health: &mut ShipHealth,
    layer_resistence: &ShipResistances,
) {
    for layer in HealthLayerType::ALL {
        // Calculate effective damage per type
        let mut applied = HealthPercents::default();
        let mut total = 0.0;
        for dmg_type in HealthChangeType::ALL {
            applied[dmg_type] = damage[dmg_type] * damage_efficiency[layer][dmg_type] * (1.0 - layer_resistence[layer][dmg_type]);
            total += applied[dmg_type];
        }

        let absorbed = total.min(ship_health.values[layer] as f32);
        ship_health.values[layer] = (ship_health.values[layer] as f32 - absorbed).min(ship_health.values_max[layer] as f32) as i32;

        // Scale applied damage to remaining fraction
        let fraction_remaining = if total > 0.0 { 1.0 - absorbed / total } else { 0.0 };
        for dmg_type in HealthChangeType::ALL {
            damage[dmg_type] = if layer_resistence[layer][dmg_type] < 1.0 {
                applied[dmg_type] * fraction_remaining / (1.0 - layer_resistence[layer][dmg_type])
            } else {
                0.0
            };
        }
    }
}

/// Processes all pending damage events for ships and applies them to their health layers.
///
/// This system reads `HealthDamageReceived` messages for the current frame and applies each
/// damage instance to the corresponding ship entity. It splits the raw damage amount into
/// a `HealthPercents` vector according to the damage profile, then calls
/// `apply_damage_vector` to reduce health across shield, armor, and hull layers while
/// respecting resistances and damage efficiencies.
///
/// # Parameters
/// - `events`: A `MessageReader` for `HealthDamageReceived` messages. Each message contains
///   the target entity, raw damage amount, damage profile (distribution per damage type),
///   and damage efficiency modifiers.
/// - `query`: A `Query` to access mutable `ShipHealth` and `ShipResistances` components for
///   the entities targeted by the damage events.
///
/// # Behavior
/// - For each damage event, the system:
///     1. Splits the total damage into per-type values based on `damage_profile`.
///     2. Calls `apply_damage_vector` to reduce the ship's health layers in order:
///        shield → armor → hull.
///     3. Automatically propagates any remaining damage from one layer to the next, scaled
///        according to resistances and efficiencies.
/// - This system only applies **damage**; healing should be handled in a separate system.
///
/// # Notes
/// - Health values are integers (`i32`), while damage, resistances, and efficiencies are
///   floating-point (`f32`).
/// - All damage events for the frame are processed sequentially, ensuring proper overflow
///   from shields to armor to hull.
pub fn apply_damage_system(
    mut events: MessageReader<HealthDamageReceived>,
    mut query: Query<(&mut ShipHealth, &ShipResistances)>,
) {
    for event in events.read() {
        if let Ok((mut health, resistances)) = query.get_mut(event.entity) {
            let damage = HealthPercents::split_value_by_percentages(event.damage, event.damage_profile);

            apply_damage_vector(damage, event.damage_efficiency, &mut health, resistances);
        }
    }
}

fn median_three(a: f32, b: f32, c: f32) -> f32 {
    let mut vals = [a, b, c];
    vals.sort_by(|x, y| x.partial_cmp(y).unwrap());
    vals[1] // middle value
}

/// Applies healing across the ship's health layers, taking into account layer-specific resistances.
///
/// Healing flows through the layers in order: **Shield → Armor → Hull**. Each layer has a maximum
/// health (`ShipHealth.values_max`) that cannot be exceeded. Healing is **scaled by specific resistances**
/// per layer:
///
/// - **Shield layer:** Healing is reduced by the ship's **Electromagnetic resistance**.
/// - **Armor layer:** Healing is reduced by **Kinetic, Thermal, and Explosive resistances**.
/// - **Hull layer:** Healing is reduced by **Kinetic, Thermal, and Explosive resistances**.
///
/// After applying healing to a layer, any leftover healing (that was capped by max health) is carried
/// over to the next layer in order. The remaining healing for each layer is recalculated after resistance
/// scaling.
///
/// # Parameters
/// - `healing`: A `HealthHealing` struct representing **raw per-layer healing amounts** (integer values).
/// - `ship_health`: Mutable reference to the ship's current and maximum health per layer (`ShipHealth`).
/// - `resistances`: A reference to the ship's resistances (`ShipResistances`) that reduce healing
///                  effectiveness per layer.
///
/// # Behavior
/// 1. For each layer in order (Shield → Armor → Hull):
///     - Identify which resistances apply to that layer.
///     - Scale the healing amount by `(1.0 - resistance)` for each applicable damage type.
///     - Sum the effective healing across the applicable damage types.
///     - Cap the healing to not exceed the layer's maximum health.
///     - Apply the capped healing to the layer.
///     - Compute any leftover healing and carry it forward to the next layer.
///
/// # Notes
/// - Healing and health are represented as `i32` values.
/// - Resistance values are `f32` in the range `[0.0, 1.0]`.
/// - Layer-specific resistances ensure that only certain damage types reduce healing for a given layer.
/// - Remaining healing is **carried forward** after each layer, preserving as much of the original
///   healing as possible.
///
/// After this, each layer's health will have increased according to the resistances and
/// capped by their maximums.
fn apply_healing_vector(
    healing: HealthHealing,
    ship_health: &mut ShipHealth,
    resistances: &ShipResistances,
) {
    let mut remaining_healing = healing;

    for layer in HealthLayerType::ALL {
        let effective_heal: i32 = match layer {
            HealthLayerType::Shield => {
                // Shield uses EM resistance
                let resist = resistances[layer][HealthChangeType::Electromagnetic];
                ((remaining_healing[layer] as f32) * (1.0 - resist)).round() as i32
            }
            HealthLayerType::Armor | HealthLayerType::Hull => {
                // Armor/Hull use median of Kinetic, Thermal, Explosive resistances
                let k = resistances[layer][HealthChangeType::Kinetic];
                let t = resistances[layer][HealthChangeType::Thermal];
                let e = resistances[layer][HealthChangeType::Explosive];
                let median = median_three(k, t, e);
                ((remaining_healing[layer] as f32) * (1.0 - median)).round() as i32
            }
        };

        // Cap healing to max health
        let current = ship_health.values[layer];
        let max = ship_health.values_max[layer];
        let healed = (effective_heal).min(max - current).max(0);
        ship_health.values[layer] += healed;

        // Remaining healing is unused (optional: could flow forward)
        // there is no overflow healing
    }
}

/// Processes all pending healing events for ships and applies them to their health layers.
///
/// This system reads `HealReceived` messages for the current frame and applies each
/// healing instance to the corresponding ship entity. It passes the raw healing amount
/// per layer to `apply_healing_vector`, which scales the healing according to the ship's
/// resistances and caps it at each layer's maximum health.
///
/// # Parameters
/// - `events`: A `MessageReader` for `HealReceived` messages. Each message contains
///   the target entity and the raw healing values for shield, armor, and hull layers.
/// - `query`: A `Query` to access mutable `ShipHealth` and `ShipResistances` components
///   for the entities targeted by the healing events.
///
/// # Behavior
/// - For each healing event, the system:
///     1. Reads the per-layer healing values from the event.
///     2. Calls `apply_healing_vector` to increase the ship's health layers in order:
///        shield → armor → hull, respecting resistances for each layer.
///     3. Ensures that healing does not exceed the maximum health for each layer.
/// - This system only applies **healing**; damage should be handled in a separate system.
///
/// # Notes
/// - Health values are integers (`i32`), while resistances are floating-point (`f32`) in
///   the range `[0.0, 1.0]`.
/// - All healing events for the frame are processed sequentially, ensuring proper capping
///   and propagation according to the resistances of each layer.
pub fn apply_heal_system(
    mut events: MessageReader<HealReceived>,
    mut query: Query<(&mut ShipHealth, &ShipResistances)>,
) {
    for event in events.read() {
        if let Ok((mut health, resistances)) = query.get_mut(event.entity) {
            let healing = event.healing;
            apply_healing_vector(healing, &mut health, resistances);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_damage_vector_basic() {
        let mut ship_health = ShipHealth {
            values: LayeredHealth { values: [10, 10, 10] },
            values_max: LayeredHealth { values: [10, 10, 10] },
        };

        let damage = HealthPercents { values: [5.0, 0.0, 0.0, 0.0] };
        let damage_efficiency = DamageEfficiency {
            values: [
                HealthPercents { values: [1.0, 0.0, 0.0, 0.0] },
                HealthPercents { values: [1.0, 0.0, 0.0, 0.0] },
                HealthPercents { values: [1.0, 0.0, 0.0, 0.0] },
            ]
        };
        let layer_resistence = ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] }, // shield
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] }, // armor
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] }, // hull
            ]
        });

        apply_damage_vector(damage, damage_efficiency, &mut ship_health, &layer_resistence);

        // Damage is applied to shield first
        assert_eq!(ship_health.values[HealthLayerType::Shield], 5);
        assert_eq!(ship_health.values[HealthLayerType::Armor], 10);
        assert_eq!(ship_health.values[HealthLayerType::Hull], 10);
    }

    #[test]
    fn test_apply_damage_vector_with_resistance() {
        let mut ship_health = ShipHealth {
            values: LayeredHealth { values: [10, 10, 10] },
            values_max: LayeredHealth { values: [10, 10, 10] },
        };

        let damage = HealthPercents { values: [10.0, 0.0, 0.0, 0.0] };
        let damage_efficiency = DamageEfficiency {
            values: [
                HealthPercents { values: [1.0, 0.0, 0.0, 0.0] },
                HealthPercents { values: [1.0, 0.0, 0.0, 0.0] },
                HealthPercents { values: [1.0, 0.0, 0.0, 0.0] },
            ]
        };
        let layer_resistence = ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0.5, 0.0, 0.0, 0.0] }, // 50% shield resistance
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] },
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] },
            ]
        });

        apply_damage_vector(damage, damage_efficiency, &mut ship_health, &layer_resistence);

        // Shield absorbs half of 10 => 5 damage
        assert_eq!(ship_health.values[HealthLayerType::Shield], 5);
        assert_eq!(ship_health.values[HealthLayerType::Armor], 10);
        assert_eq!(ship_health.values[HealthLayerType::Hull], 10);
    }

    #[test]
    fn test_apply_damage_vector_overflow_layers() {
        let mut ship_health = ShipHealth {
            values: LayeredHealth { values: [3, 5, 10] },
            values_max: LayeredHealth { values: [3, 5, 10] },
        };

        let damage = HealthPercents { values: [10.0, 0.0, 0.0, 0.0] };
        let damage_efficiency = DamageEfficiency {
            values: [
                HealthPercents { values: [1.0, 0.0, 0.0, 0.0] },
                HealthPercents { values: [1.0, 0.0, 0.0, 0.0] },
                HealthPercents { values: [1.0, 0.0, 0.0, 0.0] },
            ]
        };
        let layer_resistence = ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] },
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] },
                HealthPercents { values: [0.0, 0.0, 0.0, 0.0] },
            ]
        });

        apply_damage_vector(damage, damage_efficiency, &mut ship_health, &layer_resistence);

        // Damage overflow: 3 to shield, remaining 7 to armor (5 max), leftover 2 to hull
        assert_eq!(ship_health.values[HealthLayerType::Shield], 0);
        assert_eq!(ship_health.values[HealthLayerType::Armor], 0);
        assert_eq!(ship_health.values[HealthLayerType::Hull], 8);
    }

    #[test]
    fn test_apply_damage_vector_no_damage() {
        let mut ship_health = ShipHealth {
            values: LayeredHealth { values: [5, 5, 5] },
            values_max: LayeredHealth { values: [5, 5, 5] },
        };

        let damage = HealthPercents { values: [0.0, 0.0, 0.0, 0.0] };
        let damage_efficiency = DamageEfficiency {
            values: [
                HealthPercents::default(),
                HealthPercents::default(),
                HealthPercents::default(),
            ]
        };
        let layer_resistence = ShipResistances(LayeredHealth {
            values: [
                HealthPercents::default(),
                HealthPercents::default(),
                HealthPercents::default(),
            ]
        });

        apply_damage_vector(damage, damage_efficiency, &mut ship_health, &layer_resistence);

        // No damage applied
        assert_eq!(ship_health.values[HealthLayerType::Shield], 5);
        assert_eq!(ship_health.values[HealthLayerType::Armor], 5);
        assert_eq!(ship_health.values[HealthLayerType::Hull], 5);
    }

    #[test]
    fn test_apply_damage_vector_mixed_types() {
        let mut ship_health = ShipHealth {
            values: LayeredHealth { values: [10, 10, 10] },
            values_max: LayeredHealth { values: [10, 10, 10] },
        };

        // Mixed damage: 4 Kinetic, 6 Thermal, 2 Explosive, 8 EM
        let damage = HealthPercents { values: [4.0, 6.0, 2.0, 8.0] };

        // Damage efficiency: shield absorbs EM only, armor all physical types, hull all types
        let damage_efficiency = DamageEfficiency {
            values: [
                // Shield
                HealthPercents { values: [0.0, 0.0, 0.0, 1.0] },
                // Armor
                HealthPercents { values: [1.0, 1.0, 1.0, 0.0] },
                // Hull
                HealthPercents { values: [1.0, 1.0, 1.0, 1.0] },
            ],
        };

        // Resistances: Shield has 50% EM resistance, Armor 25% Kinetic/Thermal/Explosive, Hull no resistance
        let layer_resistance = ShipResistances(LayeredHealth {
            values: [
                HealthPercents { values: [0.0, 0.0, 0.0, 0.5] }, // shield
                HealthPercents { values: [0.25, 0.25, 0.25, 0.0] }, // armor
                HealthPercents::default(), // hull
            ],
        });

        apply_damage_vector(damage, damage_efficiency, &mut ship_health, &layer_resistance);

        // Shield: 8 EM * (1 - 0.5) = 4 damage -> 10 - 4 = 6
        assert_eq!(ship_health.values[HealthLayerType::Shield], 6);

        // Armor: 
        // Physical damage: Kinetic 4*1*(1-0.25)=3, Thermal 6*1*(1-0.25)=4.5, Explosive 2*1*(1-0.25)=1.5
        // Total = 3 + 4.5 + 1.5 = 9
        // Health = 10 - 9 = 1
        assert_eq!(ship_health.values[HealthLayerType::Armor], 1);

        // Hull: remaining damage (overflow) for each type
        // Kinetic: remaining from armor = 4 - 3 =1
        // Thermal: 6 - 4.5 = 1.5
        // Explosive: 2 -1.5 = 0.5
        // EM: nothing left (shield absorbed 4/8, remaining 4, but efficiency for hull=1, resistance=0)
        // EM remaining 4 goes to hull
        // Total hull damage = 1 + 1.5 + 0.5 + 4 = 7
        // Health = 10 - 7 = 3
        assert_eq!(ship_health.values[HealthLayerType::Hull], 3);
    }
}

#[cfg(test)]
mod table_driven_damage_tests {
    use super::*;

    struct TestCase {
        damage: HealthPercents,
        damage_efficiency: DamageEfficiency,
        layer_resistance: ShipResistances,
        expected: LayeredHealth<i32>,
        description: &'static str,
    }

    #[test]
    fn test_apply_damage_vector_table() {
        let test_cases = vec![
            TestCase {
                damage: HealthPercents { values: [10.0, 0.0, 0.0, 0.0] },
                damage_efficiency: DamageEfficiency {
                    values: [
                        HealthPercents { values: [1.0, 0.0, 0.0, 0.0] },
                        HealthPercents { values: [1.0, 0.0, 0.0, 0.0] },
                        HealthPercents { values: [1.0, 0.0, 0.0, 0.0] },
                    ],
                },
                layer_resistance: ShipResistances(LayeredHealth {
                    values: [
                        HealthPercents { values: [0.5, 0.0, 0.0, 0.0] },
                        HealthPercents { values: [0.0, 0.0, 0.0, 0.0] },
                        HealthPercents::default(),
                    ]
                }),
                expected: LayeredHealth { values: [5, 5, 10] },
                description: "Kinetic 10 with 50% shield resistance",
            },
            TestCase {
                damage: HealthPercents { values: [5.0, 5.0, 0.0, 0.0] },
                damage_efficiency: DamageEfficiency {
                    values: [
                        HealthPercents { values: [1.0, 0.0, 0.0, 0.0] },
                        HealthPercents { values: [1.0, 1.0, 0.0, 0.0] },
                        HealthPercents { values: [1.0, 1.0, 0.0, 0.0] },
                    ],
                },
                layer_resistance: ShipResistances(LayeredHealth {
                    values: [
                        HealthPercents { values: [0.0, 0.0, 0.0, 0.0] },
                        HealthPercents { values: [0.2, 0.5, 0.0, 0.0] },
                        HealthPercents::default(),
                    ]
                }),
                expected: LayeredHealth { values: [5, 1, 9] },
                description: "Mixed Kinetic/Thermal with resistances in armor",
            },
            TestCase {
                damage: HealthPercents { values: [0.0, 0.0, 5.0, 5.0] },
                damage_efficiency: DamageEfficiency {
                    values: [
                        HealthPercents { values: [0.0, 0.0, 1.0, 0.0] },
                        HealthPercents { values: [0.0, 0.0, 1.0, 0.0] },
                        HealthPercents { values: [1.0, 1.0, 0.0, 1.0] },
                    ],
                },
                layer_resistance: ShipResistances(LayeredHealth {
                    values: [
                        HealthPercents { values: [0.0, 0.0, 0.5, 0.0] },
                        HealthPercents { values: [0.0, 0.0, 0.0, 0.0] },
                        HealthPercents::default(),
                    ]
                }),
                expected: LayeredHealth { values: [8, 5, 5] },
                description: "Explosive & EM damage with partial shield resistances",
            },
        ];

        for case in test_cases {
            let mut ship_health = ShipHealth {
                values: LayeredHealth { values: [10, 10, 10] },
                values_max: LayeredHealth { values: [10, 10, 10] },
            };

            apply_damage_vector(
                case.damage,
                case.damage_efficiency,
                &mut ship_health,
                &case.layer_resistance,
            );

            for layer in HealthLayerType::ALL {
                assert_eq!(
                    ship_health.values[layer],
                    case.expected[layer],
                    "Failed test case: {} on layer {:?}",
                    case.description,
                    layer
                );
            }
        }
    }
}