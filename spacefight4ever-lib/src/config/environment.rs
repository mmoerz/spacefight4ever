use std::{env, fs, path::PathBuf};
use serde::{Serialize, Deserialize};

use bevy::prelude::*;

use directories::ProjectDirs;

/// Camera settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sf4eConfigCamera {
    pub distance_default: f32,
}

impl Default for Sf4eConfigCamera {
    fn default() -> Self {
        Self {
            distance_default: 10.0,
        }
    }
}

/// Mouse settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sf4eConfigMouse {
    pub sensitivity: f32,
    pub scroll_sensitivity: f32,
}

impl Default for Sf4eConfigMouse {
    fn default() -> Self {
        Self {
            sensitivity: 0.9,
            scroll_sensitivity: 0.95,
        }
    }
}

// the application configuration / settings that can be stored and loaded
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub asset_path: String,
    pub camera: Sf4eConfigCamera,
    pub mouse: Sf4eConfigMouse,
}

/// Event to trigger saving the config
#[derive(Message)]
pub struct SaveConfigMessage;

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            asset_path: env::var("SF4E_ASSET_PATH").unwrap_or_else(|_| "../assets".to_string()),
            camera: Sf4eConfigCamera::default(),
            mouse: Sf4eConfigMouse::default(),
        }
    }
}

/// just a helper function called by a system to load config
fn get_config_path() -> PathBuf {
    let proj_dirs = ProjectDirs::from("de", "natoka", "spacefight4ever")
        .expect("Could not determine config directory");

    let config_dir = proj_dirs.config_dir();
    std::fs::create_dir_all(config_dir).ok();

    config_dir.join("config.ron")
}

/// just a helper function called by a system to save config
fn load_config(mut commands: Commands) {
    let config_path: PathBuf = get_config_path();

    let mut config = AppConfig::default();

    // override with file if present
    if let Ok(data) = fs::read_to_string(config_path) {
        if let Ok(file_config) = ron::from_str::<AppConfig>(&data) {
            config = file_config;
        }
    }

    save_config_immediate(&config).unwrap();
    commands.insert_resource(config);
}

/// System that writes the config to disk when the event is sent
fn save_config_event_system(
    mut events: MessageReader<SaveConfigMessage>,
    config: Res<AppConfig>,
) {
    for _ in events.read() {
        save_config_immediate(&config).unwrap();
    }
}

/// just a helper function to be called by first load to ensure config is saved
fn save_config_immediate(config: &AppConfig) -> std::io::Result<()> {
    let path = get_config_path();

    let pretty = ron::ser::PrettyConfig::default();
    let data = ron::ser::to_string_pretty(config, pretty).unwrap();

    // check if file exists and matches current config
    if let Ok(existing) = fs::read_to_string(&path) {
        if existing == data {
            // nothing changed, skip writing
            return Ok(());
        }
    }

    std::fs::write(path, data)
}

pub struct ConfigPlugin;

/// Plugin for AppConfig settings load and store
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app
            // ensure a default AppConfig exists
            .init_resource::<AppConfig>()
            .add_message::<SaveConfigMessage>()
            .add_systems(Startup, load_config)
            .add_systems(Update, save_config_event_system);
    }
}