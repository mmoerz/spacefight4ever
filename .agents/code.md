**Code style example:**

```Rust
// ✅ Good - descriptive names, and comments describing intentioned usage
#[derive(Clone, Copy, Debug)]
pub enum HealthLayerType {
    Shield,
    Armor,
    Hull
}

impl HealthLayerType {
    pub fn index(self) -> usize {
        match self {
            HealthLayerType::Shield => 0,
            HealthLayerType::Armor => 1,
            HealthLayerType::Hull => 2,
        }
    }
    pub const ALL: [HealthLayerType; 3] = [
        HealthLayerType::Shield,
        HealthLayerType::Armor,
        HealthLayerType::Hull,
    ];
}

/// Generic container for per-layer values
#[derive(Debug, Default, Clone, Copy, PartialEq, Deserialize, Serialize)] 
pub struct LayeredHealth<T: Default + Copy> {
     pub values: [T; 3], 
} 

impl<T: Default + Copy> Index<HealthLayerType> for LayeredHealth<T> {
    type Output = T;
     
    fn index(&self, layer: HealthLayerType) -> &Self::Output {
        &self.values[layer.index()] 
    } 
} 

impl<T: Default + Copy> IndexMut<HealthLayerType> for LayeredHealth<T> {
    fn index_mut(&mut self, layer: HealthLayerType) -> &mut Self::Output {
        &mut self.values[layer.index()] 
    } 
}

#[derive(Component, Clone, Copy)]
pub struct ShipHealth {
    /// current health value
    pub values: LayeredHealth<i32>,
    /// current health value cannot exceed this
    pub values_max: LayeredHealth<i32>
}
```

```Rust
// ✅ Good - descriptive names, and comments describing intentioned usage
pub fn apply_damage_system(
    mut events: MessageReader<HealthDamageReceived>,
    mut query: Query<(&mut ShipHealth, &ShipResistances)>,
    mut absorbed_writer: MessageWriter<HealthDamageAbsorbed>,
) {
    for event in events.read() {
        if let Ok((mut health, resistances)) = query.get_mut(event.target) {
            let damage = HealthPercents::split_value_by_percentages(event.damage, event.damage_profile);

            let absorbed = apply_damage_vector(
                damage, event.damage_efficiency, &mut health, resistances);
            
            absorbed_writer.write(HealthDamageAbsorbed {
                entity: event.target,
                damage: absorbed
            });
        }
    }
}
```

```Rust
// ❌ Bad - vague names, no empty line after local variable definition
fn get(x: int) {
  let mut z = x*x +1
  let y = x+4
  z*y
}
```
