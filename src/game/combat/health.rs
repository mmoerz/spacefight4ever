use bevy::ecs::component::Component;

#[derive(Component)]
pub struct ShipHealth {
    pub shield: i32,
    pub shield_max: i32,
    pub armor: i32,
    pub armor_max: i32,
    pub hull: i32,
    pub hull_max: i32,
}