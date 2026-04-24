use std::ops::{Index, IndexMut};

use bevy::prelude::*;

#[derive(Message)]
pub struct Attack {
    pub attacker: Entity,
    pub target: Entity,
    pub weapon: String,
}

pub fn calculate_attack(

) {

}

// pub fn attack_system(
//     mut events: MessageReader<Attack>,
//     mut query: Query<(&mut Ship, &ShipModel, &ShipResistances)>,
// ) {
//     for ev in events.read() {
        
//     }
// }