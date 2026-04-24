use bevy::prelude::*;

pub trait Stat {
    fn set(&mut self, value: f32);
    fn get(&self) -> f32;
    fn normalize(&self) -> f32;
    fn min(&self) -> f32;
    fn max(&self) -> f32;
    fn set_min(&mut self, value: f32);
    fn set_max(&mut self, value: f32);
}

/// hmm this would probably defeat the ecs pattern

/// Stat storage
/// this is a storage for the final stat values
#[derive(Component, Default, Debug, Clone)]
pub struct Stats {
    pub values: std::collections::HashMap<StatKey, f32>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum StatKey {
    // propulsion related
    Speed,
    Thrust,

    // defense related
    Shield,
    Armor,
    Hull,

    // energy related
    Energy,
    Heat,
}

/// minimum and maximum values for a stat
#[derive(Component, Debug, Clone)]
pub struct StatLimits {
    pub min: f32,
    pub max: f32,
}

/// storage for min and max of a stat
#[derive(Component, Default)]
pub struct StatBounds {
    pub bounds: std::collections::HashMap<StatKey, StatLimits>,
}

pub fn get_normalized(
    stats: &Stats,
    bounds: &StatBounds,
    key: StatKey,
) -> f32 {
    let value = stats.values.get(&key).copied().unwrap_or(0.0);
    let limit = bounds.bounds.get(&key);

    if let Some(limit) = limit {
        (value - limit.min) / (limit.max - limit.min)
    } else {
        value // assume already normalized
    }
}

// TODO: do  not leeave here 

// #[derive(Component)]
// pub struct UiStatBar {
//     pub entity: Entity,
//     pub stat: StatKey,
// }

// pub fn ui_stat_bar_system(
//     query: Query<(&UiStatBar, &UiProgressBarHandle)>,
//     ships: Query<(&Stats, &StatBounds)>,
//     mut materials: ResMut<Assets<UiProgressBarMaterial>>,
// ) {
//     for (ui, handle) in &query {
//         if let Ok((stats, bounds)) = ships.get(ui.entity) {
//             let value = get_normalized(stats, bounds, ui.stat.clone());

//             if let Some(mat) = materials.get_mut(&handle.0) {
//                 mat.set(value);
//             }
//         }
//     }
// }

// commands.spawn((
//     UiStatBar {
//         entity: ship_entity,
//         stat: StatKey::Speed,
//     },
//     UiProgressBarHandle(material_handle),
// ));