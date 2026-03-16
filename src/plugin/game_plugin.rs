// for sure to much will end up here, so split it up later ...

use bevy::prelude::*;

use crate::game::player::player::*;
use crate::game::combat::health::*;

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
        app.add_systems(
                Update,
                (
                    apply_damage_system,
                    apply_heal_system.after(apply_damage_system),
                ),
            );
    }
}