# Architecture
I'm going to describe my design decission here.

## Folder Structure

```
src/
  main.rs
  plugins/
  game/
     combat/
     player/
     world/
  ui/
     mod.rs
     state.rs
     events.rs
     roots.rs
     widgets/
        button.rs
        dialog.rs
        tooltip.rs
     systems/
        dialog_system.rs
        menu_system.rs
        hud_system.rs
```

### Layered Architecture
```
Game Logic Layer
   ↓ events
UI State Layer
   ↓ systems
UI Rendering Layer
   ↓
Bevy UI / egui widgets
```

