# current architecture

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

# Ideas for improvements

## Asset‑Backed Dialog Factory (Hybrid)
1) write templates in e.g. Ron>
```
assets/ui/confirm_exit.ron
assets/ui/message_dialog.ron
```

2) At startup parse templates
```
let templates = DialogTemplates::load_from_dir("assets/ui");
commands.insert_resource(templates);
```

3) Spawn Dialogs from templates
```
if let Some(dialog) = stack.top() {
    ui_templates.spawn(dialog.clone(), &mut commands, &asset_server);
}
```

4) if using vector graphics assets
refence them in template and use bevy_svg to load thme

