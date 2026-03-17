use bevy::prelude::*;

/// the 'hardpoint' type
/// there are different types of slots where 
/// modules can be equipped, but only if the
/// module can be mounted into that hardpoint
/// type
pub enum HardPointType {
    Weapon,
    Shield,
    Armor,
    Support,
    Propulsion
}

/// a 'hardpoint' where modules can be equiped
/// 
pub struct HardPoint {
    pub hardpoint_type: HardPointType,
}

/// a simple ship modul that a ship can equip
/// the slot_type limits the modul to that type
/// so that it cannot be mounted in not corresponding
/// slots
pub struct ShipModul {
    pub name: String,
    pub slot_type: HardPointType
}