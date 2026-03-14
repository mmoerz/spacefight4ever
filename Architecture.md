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


fixed ui Root nodes
```
UiRoot
 ├ HUDRoot
 ├ MenuRoot
 ├ DialogRoot
 └ OverlayRoot
```

### Ui State
Instead of spawning/despawning UI randomly, UI is controlled by state resources.

Example:

```
#[derive(Resource)]
struct UiState {
    pause_menu: bool,
    inventory_open: bool,
    dialog_stack: Vec<Dialog>,
}
```
Systems read this state and update the UI.

This pattern gives you:

    deterministic UI

    easier debugging

    easier save/load



### event driven ui system
```
enum UiEvent {
    OpenInventory,
    ShowDialog(Dialog),
    ShowNotification(String),
}
```

Flow:
```
Gameplay system
    ↓
UiEvent
    ↓
UI systems
    ↓
spawn/update UI
```