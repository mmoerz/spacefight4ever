use bevy::prelude::*;

use crate::game::ship::ships;

#[derive(Message)]
pub struct Attack {
    pub attacker: Entity,
    pub target: Entity,
    pub weapon: String,
}

pub fn attack_system(
    mut events: MessageReader<Attack>,
    mut query: Query<(&mut Ship, &ShipModel, &ShipResistances)>,
) {

}