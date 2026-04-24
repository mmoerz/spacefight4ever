use std::str::FromStr;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};


use crate::game::ship::definitions::module_definition::{Module, ModuleSize};

/// the 'hardpoint' type
/// there are different types of slots where 
/// modules can be equipped, but only if the
/// module can be mounted into that hardpoint
/// type
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum HardPointType {
    #[default]
    Weapon,
    Shield,
    Armor,
}

/// the 'slot' type
/// for support and generic modules
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum SlotType {
    #[default]
    Support,
    Propulsion
}

/// the 'mountpoint' type
/// hardpoint is for weapons and fixed in orientation
/// slot is for internal & support modules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum MountType {
    Hardpoint(HardPointType), // directional, combat-oriented
    Slot(SlotType), // general purpose
}

impl FromStr for MountType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HardpointWeapon" => Ok(MountType::Hardpoint(HardPointType::Weapon)),
            "HardpointShield" => Ok(MountType::Hardpoint(HardPointType::Shield)),
            "HardpointArmor" => Ok(MountType::Hardpoint(HardPointType::Armor)),
            "SlotSupport" => Ok(MountType::Slot(SlotType::Support)),
            "SlotPropulsion" => Ok(MountType::Slot(SlotType::Propulsion)),
            _ => Err(()),
        }
    }
}

impl Default for MountType {
    fn default() -> Self {
        MountType::Hardpoint(HardPointType::Weapon)
    }
}

/// a 'hardpoint' where modules can be equiped
/// 
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MountPoint {
    pub id: u32,
    pub kind: MountType,
    pub allowed_size: ModuleSize,
    pub occupied: Option<Entity>,
}

impl MountPoint {
    /// check wether the mountpoint can equip the module or not
    pub fn can_equip(&self, module: &Module) -> bool {
        // Check mount type compatibility
        let type_ok = match (&self.kind, &module.kind) {
            (MountType::Hardpoint(a), 
             MountType::Hardpoint(b)) => a == b,
            (MountType::Slot(a), MountType::Slot(b)) => a == b,
            _ => false,
        };

        type_ok && self.allowed_size == module.size
    }
}

pub struct MountPointBuilder {
    pub id: u32,
    pub kind: MountType,
    pub allowed_size: ModuleSize,
    pub occupied: Option<Bundle>,
}

impl MountPointBuilder {
    pub fn new(id: u32, kind: MountType, allowed_size: ModuleSize) -> Self {
        Self {
            id,
            kind,
            allowed_size,
            occupied: None,
        }
    }

    pub fn set_module(self, bundle: Bundle) -> Self {
        Self {
            occupied: Some(bundle),
            ..self
        }
    }

    fn build(
        self,
        mut commands: Commands,
    ) -> Entity {
        let mountpoint =  
            commands.spawn(
                MountPoint {
                    id: self.id,
                    kind: self.kind,
                    allowed_size: self.allowed_size,
                    occupied: self.occupied,
                }
            ).id();
        if let Some(occupied) = self.occupied {
            commands.entity(mountpoint).add_child(occupied);
        }
        mountpoint
    }
}
