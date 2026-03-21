# ship - module - weapon relationships

```
[Ship Entity]
 ├─ MountPoint[0] 
 |  └─ WeaponModuleBundle (Laser)
 │     └─ Weapon
 |     └─ Ammunition (Laser Ammo)
 |        Ammunition.count
 ├─ MountPoint[1] 
 │  └─ WeaponModuleBundle (Missile Launcher)
 │     └─ Weapon 
 │     └─ Ammunition (Missiles)
 └─ MountPoint[2] 
    └─ WeaponModuleBundle (Railgun)
        └─ Weapon
        └─ Ammunition (Railgun Ammo)
```

