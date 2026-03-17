use std::env;

// some basic config struct
pub struct Sf4eConfig {
    pub asset_path: String,
}

/// fetch configuration from environment variables
/// This includes:
/// SF4E_ASSET_PATH
pub fn get_s4fe_config() -> Sf4eConfig {
    Sf4eConfig {
        // default is development environment
        asset_path: env::var("SF4E_ASSET_PATH").unwrap_or_else(|_| "../assets".to_string()),
    }
}