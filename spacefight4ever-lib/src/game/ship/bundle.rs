use bevy::prelude::*;

use super::module::*;
use super::weapon::*;

// Example: Put this in game/ship/weapon.rs or similar
#[derive(Bundle)]
pub struct WeaponModuleBundle {
    pub module: Module,   // The generic fitting info
    pub weapon: Weapon,   // The specific combat info
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl WeaponModuleBundle {
    pub fn new(module: Module, weapon: Weapon) -> Self {
        Self {
            module,
            weapon,
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}