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

# crates in use
  - bevy_ui_widgets - buttons, sliders, checkboxes
  - not in use, not compatible with bevy 0.18 bevy_ui_material - ready to use bundles, defaults (background, padding)