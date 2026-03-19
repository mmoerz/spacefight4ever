use bevy::prelude::*;

/// the 'hardpoint' type
/// there are different types of slots where 
/// modules can be equipped, but only if the
/// module can be mounted into that hardpoint
/// type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardPointType {
    Weapon,
    Shield,
    Armor,
}

/// the slot type
/// for support and generic modules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotType {
    Support,
    Propulsion
}

/// the 'mountpoint' type
/// hardpoint is for weapons and fixed in orientation
/// slot is for internal & support modules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountType {
    Hardpoint(HardPointType), // directional, combat-oriented
    Slot(SlotType), // general purpose
}

/// a 'hardpoint' where modules can be equiped
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MountPoint {
    id: usize,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModuleSize {
    Micro,
    Small,
    Medium,
    Large,
    XLarge
}

/// a simple ship modul that a ship can equip
/// the slot_type limits the modul to that type
/// so that it cannot be mounted in not corresponding
/// slots
#[derive(Component)]
pub struct Module {
    pub id: usize,
    pub name: String,
    pub kind: MountType,
    pub size: ModuleSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShipType {
    Fighter,
    Bomber,
    Corvette,
    Frigate,
    Destroyer,
    Cruiser,
    Battlecruiser,
    Battleship,
    Carrier
}

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct ShipModel {
    pub name: String,
    pub kind: ShipType,
    pub manufacturer: String,
    pub mount_points: Vec<MountPoint> // default (empty mountpoints)
    // put a reference or identifier for the graphics part here?
}

#[derive(Component)]
pub struct Ship {
    pub id: usize,
    pub kind: ShipType,
    pub mount_points: Vec<MountPoint>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mountpoint_can_equip() {
        // Example mount points
        let weapon_hardpoint = MountPoint {
            id: 1,
            kind: MountType::Hardpoint(HardPointType::Weapon),
            allowed_size: ModuleSize::Small,
            occupied: None,
        };

        let shield_hardpoint = MountPoint {
            id: 2,
            kind: MountType::Hardpoint(HardPointType::Shield),
            allowed_size: ModuleSize::Medium,
            occupied: None,
        };

        let support_slot = MountPoint {
            id: 3,
            kind: MountType::Slot(SlotType::Support),
            allowed_size: ModuleSize::Large,
            occupied: None,
        };

        // Example modules
        let laser_module = Module {
            id: 100,
            name: "Laser Cannon".to_string(),
            kind: MountType::Hardpoint(HardPointType::Weapon),
            size: ModuleSize::Small,
        };

        let shield_module = Module {
            id: 101,
            name: "Shield Generator".to_string(),
            kind: MountType::Hardpoint(HardPointType::Shield),
            size: ModuleSize::Medium,
        };

        let support_module = Module {
            id: 102,
            name: "Cargo Bay".to_string(),
            kind: MountType::Slot(SlotType::Support),
            size: ModuleSize::Large,
        };

        let wrong_size_module = Module {
            id: 103,
            name: "Oversized Laser".to_string(),
            kind: MountType::Hardpoint(HardPointType::Weapon),
            size: ModuleSize::Medium,
        };

        // ✅ Positive tests
        assert!(weapon_hardpoint.can_equip(&laser_module));
        assert!(shield_hardpoint.can_equip(&shield_module));
        assert!(support_slot.can_equip(&support_module));

        // ❌ Negative tests
        assert!(!weapon_hardpoint.can_equip(&shield_module)); // wrong hardpoint type
        assert!(!weapon_hardpoint.can_equip(&wrong_size_module)); // wrong size
        assert!(!support_slot.can_equip(&laser_module)); // wrong slot type
    }
}