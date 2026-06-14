# Architecture
I'm going to describe my design decission here.

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

## Current Implementation Status

The project follows a layered architecture approach where:

1. **Game Logic Layer** - Contains core gameplay systems and logic in `spacefight4ever-lib/src/game/` and `spacefight4ever-lib/src/plugin/`
2. **UI State Layer** - Manages UI state through resources and events in `spacefight4ever-lib/src/ui/`
3. **UI Rendering Layer** - Implements Bevy UI components and rendering systems in `crates/spacefight4ever_ui/`

Plugins of `spacefight4ever-lib` are organized as follows:
- `UiPlugin` in `spacefight4ever-lib/src/plugin/ui_plugin.rs`
   functionality resides in `spacefight4ever-lib/src/ui`
- `CombatPlugin` in `spacefight4ever-lib/src/plugin/game_plugin.rs`
   functionality resides in `spacefight4ever-lib/src/game`
- `PlayerPlugin` in `spacefight4ever-lib/src/game/player/player.rs`
   functionality resides in throughout the code - maybe fix that?

This structure allows for clear separation of concerns while following Bevy's Plugin architecture.
