use bevy::prelude::*;

/// each transportable (cargo) item consumes space and weight
#[derive(Component)]
pub struct CargoItem {
    pub volume: f32,
    pub weight: f32,
}

#[derive(Component)]
pub struct Cargo {
    pub items: Vec<Entity>,
}