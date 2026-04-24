use bevy::prelude::*;

use crate::game::ship::definitions::module_definition::ModuleDefinition;

// impl Module {
//     /// Helper to create a standard weapon module definition
//     pub fn new_weapon(id: u32, name: impl Into<String>, size: ModuleSize) -> Self {
//         Self {
//             id,
//             name: name.into(),
//             kind: MountType::Hardpoint(HardPointType::Weapon),
//             size,
//         }
//     }
// }

// a simple ship modul that a ship can equip
// the slot_type limits the modul to that type
// so that it cannot be mounted in not corresponding
// slots
#[derive(Component)]
pub struct Module {
    /// id for hardpoint / slot of the ship
    pub id: u32,
    pub handle: Handle<ModuleDefinition>,
//     pub id: u32,
//     pub name: String,
//     pub kind: MountType, /// this is the equivalent to the module type (weapon, shield, ...)
//     pub size: ModuleSize,
}






// // each ship has a list of mountpoints that adhere
// // to the 'default' mountpoints defined in ShipModel
// #[derive(Component)]
// pub struct Ship {
//     pub id: u32,
//     pub kind: ShipType,
//     //pub mount_points: Vec<MountPoint> -- directly attached in the scene graph
// }

// impl Ship {
//     // Logically assigns a module entity to a specific mount point ID.
//     //
//     // Note: This does not handle the hierarchy (parent/child) relationship.
//     // That has to be done in a bundle.
//     // pub fn equip(&mut self, mount_point_id: u32, module: &Module) -> Result<(), String> {
//     //     if let Some(mp) = self.mount_points.iter_mut().find(|mp| mp.id == mount_point_id) {
//     //         if mp.occupied.is_some() {
//     //             return Err(format!("Mount point {} is already occupied", mount_point_id));
//     //         }
//     //         mp.occupied = Some(module_entity);
//     //         Ok(())
//     //     } else {
//     //         Err(format!("Mount point {} not found", mount_point_id))
//     //     }
//     // }
// }

