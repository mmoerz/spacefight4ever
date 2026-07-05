**Code style example:**

```Rust
// ✅ Good - descriptive names, and comments describing intentioned usage
#[derive(Component, Clone, Copy)]
pub struct ShipHealth {
    /// current value
    pub values: LayeredHealth<i32>,
    /// current value cannot exceed this
    pub values_max: LayeredHealth<i32>
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
