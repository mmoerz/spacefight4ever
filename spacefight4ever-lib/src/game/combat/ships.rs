use std::ops::{Deref, DerefMut};
use bevy::prelude::*;

use crate::game::combat::health_basetypes::*;

pub enum Standing {
    Enemy,
    Neutral,
    Ally
}

#[derive(Component)]
pub enum Character {
    PC,
    NPC
}

#[derive(Component)]
pub struct Ship {
    pub standing: Standing,
    
}


#[derive(Component, Debug, Default, Clone, Copy)]
pub struct ShipResistances(pub LayeredHealth<HealthPercents>);

impl Deref for ShipResistances {
    type Target = LayeredHealth<HealthPercents>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for ShipResistances {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
