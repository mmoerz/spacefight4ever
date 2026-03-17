# spacefight4ever
a bevy based space shooter

# setting up your rust
if cargo run fails complaining about a missing rust 1.89, check if there are not two toolchains in use:
```
which rustc
which cargo
type -a rustc
```
Something like:
```
/usr/bin/rustc    ← old 1.88
/home/mmoerz/.cargo/bin/rustc   ← rustup 1.93
```
shows a discrepancy

Either install all local rust version (*cleaner*) or set PATH: 
```
export PATH="$HOME/.cargo/bin:$PATH"
```

## libraries needed
ubuntu/mint>
```
apt-get install librust-wayland-client-dev libasound2-dev libudev-dev
```

# additional tools to look into
https://ast-grep.github.io/blog/migrate-bevy.html
```
# install the binary `ast-grep`
cargo install ast-grep
# or use brew
brew install ast-grep
```

# crates in use
  - bevy_ui_widgets - buttons, sliders, checkboxes
  - not in use, not compatible with bevy 0.18 bevy_ui_material - ready to use bundles, defaults (background, padding)



## even better dialogs with templates

✅ Summary of Recommended Architecture

DialogStack + DialogRequest/Result: remain the source of truth.

Dialog Spawn System: generic, spawns a dialog from a template.

Asset Templates / Prefabs: store backgrounds, frames, and button images.

Procedural Text + Buttons: dynamically inserted children for content.

Interaction System: remains the same (button presses → messages).

### prefab
#[derive(Resource)]
struct DialogTemplates {
    confirm_exit_bg: Handle<Image>,
    message_bg: Handle<Image>,
    yes_button: Handle<Image>,
    no_button: Handle<Image>,
}
fn spawn_dialog(
    commands: &mut Commands,
    dialog_type: &DialogType,
    templates: &Res<DialogTemplates>,
) {
    match dialog_type {
        DialogType::ConfirmExit => {
            commands.spawn((
                NodeBundle { /* use templates.confirm_exit_bg as image */ },
                DialogEntity,
            ));
            // Spawn procedural buttons/text as children
        }
        ...
    }
}

Create an “Asset Dialog Factory”
assets/dialogs/
 ├─ confirm_exit.svg
 ├─ confirm_exit_buttons.svg
 ├─ message_dialog.svg


 # ui editors

 https://github.com/jakobhellermann/bevy_editor_pls


 # testing 
 https://chadnauseam.com/coding/gamedev/automated-testing-in-bevy