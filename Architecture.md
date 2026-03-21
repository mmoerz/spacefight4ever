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

### Some rules
* is it ECS-specific?
   - yes, keep it with ECS
   - no, extract
* Does it operate purely on data structs?
   - yes, extractable utility
   - no, keep it local
* used in more than 1 system?
   - yes, extract
   - no, keep it local

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

