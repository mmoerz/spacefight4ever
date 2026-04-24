
use serde::{Deserialize, Serialize};

use crate::game::ship::modules::mountpoint::{MountPoint, MountType};
use crate::game::ship::definitions::module_definition::ModuleSize;

/// a 'hardpoint' where modules can be equiped
/// 
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct MountPointDefinition {
    pub id: u32,
    pub kind: MountType,
    pub allowed_size: ModuleSize,
}