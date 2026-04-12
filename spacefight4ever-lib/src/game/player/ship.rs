use bevy::prelude::*;

#[derive(Component)]
pub struct Ship {
    pub acceleration_max : f32,
    pub velocity: Vec3,
    pub max_speed: f32,
    pub acceleration: f32,
    pub turn_speed: f32,
}
