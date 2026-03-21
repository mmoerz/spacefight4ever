// for sure to much will end up here, so split it up later ...

use bevy::prelude::*;

use crate::game::player::player::*;
use crate::game::combat::health::*;
use crate::game::combat::health_basetypes::*;
use crate::game::physics::raycast_damage::*;
use crate::game::ship::weapon_definition::*;
use crate::game::ship::ammunition_definitions::*;

/// game plugin
/// currently only contains player spawn
/// might 
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player);

            
    }
}


pub struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<HealthDamageAbsorbed>()
            .add_message::<HealthDamageReceived>()
            .add_message::<HealthHealAbsorbed>()
            //.add_message::<HealthHealRequest>()
            .add_message::<WeaponFireRequest>()

            .add_systems(Startup, setup_weapon_repo)
            .add_systems(Startup, setup_ammunition_repo)

            .add_systems(
                Update,
                (
                    apply_damage_system,
                    apply_heal_system.after(apply_damage_system),
                ),
            )
            .add_systems(Update, weapon_fire_system);
    }
}