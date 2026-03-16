
# ECS pattern
Attack systems
      ↓
Damage calculation
      ↓
HealthDamageReceived messages
      ↓
apply_damage_system
      ↓
ShipHealth updated
      ↓
UI system reads ShipHealth


## Health Change

The idea is to have a health change (dmg or heal) that has an eficiency profil.
This 