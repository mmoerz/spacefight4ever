use bevy::prelude::*;

#[derive(Component)]
pub struct EnemyShip;

#[derive(Component)]
pub struct NeutralShip;

pub struct Resistances {
    kinetic: f32,
    thermal: f32,
    explosive: f32,
    electromagnetic: f32,
}

#[derive(Component)]
pub struct ShipResistances {
    pub shield_resistances : Resistances,
    pub armor_resistances : Resistances,
    pub hull_resistances : Resistances,    
}