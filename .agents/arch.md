# Architecture
my design decission are described here.

### Layered Architecture
```
Game Logic Layer
   ↓ events
UI State Layer
   ↓ systems
UI Rendering Layer
   ↓
Bevy UI
```

## Current Implementation Status

- **Layered Architecture:** Adhere strictly to the layered architecture:
  - **Game Logic Layer:** Independent of UI, communicates via events.
  - **UI State Layer:** Listens to game events and updates UI state.
  - **UI Rendering Layer:** Bevy UI components and systems.
- **Modularity:** Maintain separation between:
  - `spacefight4ever-lib` (core logic)
  - `spacefight4ever-ui` (core UI functionality)
  - `spacefight4ever-bin` (entry point)
  - `spacefight4ever-test` (verification - integration tests)
- Architecture is described in `agents/arch.md`

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
