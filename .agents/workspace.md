spacefight4ever/
├── spacefight4ever-bin/          # Binary crate — App bootstrap, plugin wiring, test setup
├── spacefight4ever-lib/          # Core game library — ECS components, systems, plugins
│   └── src/
│       ├── config.rs / config/   # App configuration (environment, asset paths)
│       ├── game/                  # Game domain
│       │   ├── assets.rs         # GameAssetsPlugin
│       │   ├── combat/           # Combat systems
│       │   ├── physics/          # Physics systems
│       │   ├── player/           # PlayerPlugin
│       │   ├── ship/             # Ship domain
│       │   └── world/            # World systems
│       ├── plugin/               # Shared plugin utilities
│       └── ui/                   # In-game UI
│           ├── camera.rs         # GameCameraPlugin
│           ├── hud/              # HUD components (movement display)
│           └── overlay/          # Overlay UI (settings, sliders)
├── crates/                       # Additional crates
│   └── spacefight4ever_ui/       # Reusable UI components & themes
└── spacefight4ever-test/         # Test crate
