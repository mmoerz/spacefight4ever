use bevy::prelude::*;
use avian3d::prelude::*;

use crate::game::ship::weapon::*;

/// system that simulates flying missiles
/// 
/// TODO: check if missile has fuel left
/// TODO: check if something is inbetween missile and target, if so stear clear from bocking object and resume course to target
/// TODO: check if missile impacts target and generate HealthDamageReceived message
pub fn missile_flying_system(
) {

}
