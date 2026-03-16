use std::ops::{Deref, DerefMut};
use bevy::prelude::*;

use crate::game::combat::health_basetypes::*;


#[derive(Component)]
pub struct EnemyShip;

#[derive(Component)]
pub struct NeutralShip;


#[derive(Component, Debug, Default, Clone, Copy)]
pub struct ShipResistances(pub Layered<HealthPercents>);

impl Deref for ShipResistances {
    type Target = Layered<HealthPercents>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for ShipResistances {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}
