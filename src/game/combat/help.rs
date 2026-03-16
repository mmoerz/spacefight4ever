use std::ops::{Index, IndexMut}; 

#[derive(Clone, Copy, Debug)]
pub enum HealthLayerType
{ Shield, Armor, Hull, }
 
impl HealthLayerType {
    pub fn index(self) -> usize {
        match self { HealthLayerType::Shield => 0, HealthLayerType::Armor => 1, HealthLayerType::Hull => 2, } 
    }
        
    pub const ALL: [HealthLayerType; 3] = [ HealthLayerType::Shield, HealthLayerType::Armor, HealthLayerType::Hull, ]; 
} 

/// Generic container for per-layer values
#[derive(Debug, Default, Clone, Copy)] 
pub struct Layered<T: Default + Copy> {
     pub values: [T; 3], 
} 

impl<T: Default + Copy> Index<HealthLayerType> for Layered<T> {
    type Output = T;
     
    fn index(&self, layer: HealthLayerType) -> &Self::Output {
        &self.values[layer.index()] 
    } 
} 

impl<T: Default + Copy> IndexMut<HealthLayerType> for Layered<T> {
    fn index_mut(&mut self, layer: HealthLayerType) -> &mut Self::Output {
        &mut self.values[layer.index()] 
    } 
} 

// Now you can define specific types:

#[derive(Debug, Default, Clone, Copy)] 
pub struct HealthPercents { 
    pub kinetic: f32, 
    pub thermal: f32, 
    pub explosive: f32, 
    pub electromagnetic: f32, 
}

pub type ShipResistances = Layered<HealthPercents>;
pub type DamageEfficiency = Layered<HealthPercents>;





// fn apply_layer_health_change(
//     mut dmg: &HealthVector,
//     health: &mut i32,
//     resist: &HealthVector,
//     effect: &HealthVector,
// ) -> HealthVector {
//     // transform damage for this layer
//     let mut transformed = HealthVector::default();

//     for t in HealthType::ALL_DAMAGE_TYPES {
//         transformed[t] = dmg[t] * effect[t] * (1.0 - resist[t]);
//     }

//     let total: f32 = transformed.values.iter().sum();

//     if total <= 0.0 {
//         return HealthVector::default();
//     }

//     // how much this layer can absorb
//     let absorbed = total.min(*health as f32);

//     // fraction absorbed
//     let f = absorbed / total;

//     *health -= absorbed as i32;

//     // remaining transformed damage
//     for t in HealthType::ALL_DAMAGE_TYPES {
//         transformed[t] *= 1.0 - f;
//     }

//     // reverse transform → back to base damage vector
//     for t in HealthType::ALL_DAMAGE_TYPES {
//         let factor = effect[t] * (1.0 - resist[t]);
//         if factor > 0.0 {
//             dmg[t] = transformed[t] / factor;
//         } else {
//             dmg[t] = 0.0;
//         }
//     }

//     dmg
// }


// fn apply_layer_health_change (
//     mut dmg: HealthVector,
//     layer_health: &mut i32,
//     dmg_effectivity: &HealthVector,
//     layer_resist: &HealthVector,
// ) -> HealthVector {
//     // transform damage by resistances and effectivity
//     let mut transformed = HealthVector::default();
//     let mut total: f32 = 0.0;

//     for t in HealthType::ALL_DAMAGE_TYPES {
//         transformed[t] = dmg[t] * dmg_effectivity[t] * (1.0 - layer_resist[t]);
//         total += transformed[t];
//     }

//     if total <= 0.0 { return HealthVector::default(); }

//     let absorbed = total.min(*layer_health as f32);
//     let f = absorbed / total;
//     *layer_health -= absorbed as i32;

//     // remaining transformed damage
//     for t in HealthType::ALL_DAMAGE_TYPES {
//         transformed[t] *= 1.0 - f;
//     }

//     // reverse to get remaining damage vector for next layer
//     for t in HealthType::ALL_DAMAGE_TYPES {
//         let factor = dmg_effectivity[t] * (1.0 - layer_resist[t]);
//         dmg[t] = if factor > 0.0 { transformed[t] / factor } else { 0.0 };
//     }

//     dmg
// }


// pub fn apply_damage(
//     mut dmg: HealthVector,
//     health: &mut ShipHealth,
//     dmg_effectivity: &HealthLayerPercent,
//     resist: &ShipResistances,
// ) {

//     dmg = apply_layer_health_change(
//         dmg,
//         &mut health.shield,
//         &dmg_effectivity.values[HealthLayerType::shield],
//         &resist.shield_resistances
//     );

//     dmg = apply_layer_health_change(
//         dmg,
//         &mut health.armor,
//         &dmg_effectivity.armor_effectivity,
//         &resist.armor_resistances
//     );

//     dmg = apply_layer_health_change(
//         dmg,
//         &mut health.hull,
//         &dmg_effectivity.hull_effectivity,
//         &resist.hull_resistances,
//     );
// }