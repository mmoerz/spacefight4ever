// for sure to much will end up here, so split it up later ...

use bevy::prelude::*;

use crate::game::player::player::*;

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