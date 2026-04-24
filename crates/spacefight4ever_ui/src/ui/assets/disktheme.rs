use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};

use crate::ui::{assets::atlasbuttonskin::UiButtonTypesAllHandles, button::{UiButtonType, UiWindowType}};
use super::asseterror::UiAssetLoadError;
use super::atlasbuttonskin::{DiskButtonSkin};
use super::theme::*;
use super::windowsskin::{WindowSkin, DiskWindowSkin};

#[derive(TypePath, Debug, Deserialize, Serialize)]
pub struct DiskUiTheme {
    pub button_skins: HashMap<String, DiskButtonSkin>,
    pub window_skins: HashMap<String, DiskWindowSkin>,
    pub colors: HashMap<String, [f32; 4]>, // RGBA
}

impl DiskUiTheme {
    /// Convert a DiskUiTheme into a fully loaded UiTheme asset
    pub fn into_runtime(
        self,
        load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<UiTheme, UiAssetLoadError> {
        // Load all button skins
        let mut button_skins = UiButtonTypesAllHandles{ types: [Handle::default(),  Handle::default(), Handle::default(), Handle::default()] };
        for (name, disk_button) in self.button_skins {
            let skin = disk_button.into_runtime(load_context)?;
            let handle = load_context.add_labeled_asset(format!("button_skin_{}", name), skin);

            if let Ok(button_type) = name.parse::<UiButtonType>() {
                button_skins[button_type] = handle;
            } else {
                warn!("Unknown button type `{name}` in theme, skipping");
            }
        }

        // Load all window skins
        let mut window_skins: HashMap<UiWindowType, Handle<WindowSkin>> = HashMap::new();
        for (name, disk_window) in self.window_skins {
            let skin = disk_window.into_runtime(load_context)?;
            let handle = load_context.add_labeled_asset(format!("window_skin_{}", name), skin);

            if let Ok(window_type) = name.parse::<UiWindowType>() {
                window_skins.insert(window_type, handle);
            } else {
                warn!("Unknown button type `{name}` in theme, skipping");
            }
        }

        // Convert colors
        let colors: HashMap<String, Color> = self
            .colors
            .into_iter()
            .map(|(k, rgba)| (k, Color::srgba(rgba[0], rgba[1], rgba[2], rgba[3])))
            .collect();

        // Create runtime UiTheme
        Ok(UiTheme {
            mode: ThemeMode::Dark, // default, can be overwritten later
            color_scheme: ColorScheme::Default,
            button_skins,
            window_skins,
            primary: colors.get("primary").cloned().unwrap_or(hex("#D0BCFF")),
            on_primary: colors.get("on_primary").cloned().unwrap_or(hex("#381E72")),
            primary_container: colors.get("primary_container").cloned().unwrap_or(hex("#4F378B")),
            on_primary_container: colors.get("on_primary_container").cloned().unwrap_or(hex("#EADDFF")),

            secondary: colors.get("secondary").cloned().unwrap_or(hex("#CCC2DC")),
            on_secondary: colors.get("on_secondary").cloned().unwrap_or(hex("#332D41")),
            secondary_container: colors.get("secondary_container").cloned().unwrap_or(hex("#4A4458")),
            on_secondary_container: colors.get("on_secondary_container").cloned().unwrap_or(hex("#E8DEF8")),

            tertiary: colors.get("tertiary").cloned().unwrap_or(hex("#EFB8C8")),
            on_tertiary: colors.get("on_tertiary").cloned().unwrap_or(hex("#492532")),
            tertiary_container: colors.get("tertiary_container").cloned().unwrap_or(hex("#633B48")),
            on_tertiary_container: colors.get("on_tertiary_container").cloned().unwrap_or(hex("#FFD8E4")),

            error: colors.get("error").cloned().unwrap_or(hex("#F2B8B5")),
            on_error: colors.get("on_error").cloned().unwrap_or(hex("#601410")),
            error_container: colors.get("error_container").cloned().unwrap_or(hex("#8C1D18")),
            on_error_container: colors.get("on_error_container").cloned().unwrap_or(hex("#F9DEDC")),

            surface: colors.get("surface").cloned().unwrap_or(hex("#121212")),
            on_surface: colors.get("on_surface").cloned().unwrap_or(hex("#E6E1E5")),
            on_surface_variant: colors.get("on_surface_variant").cloned().unwrap_or(hex("#CAC4D0")),
            surface_container_lowest: colors.get("surface_container_lowest").cloned().unwrap_or(hex("#121212")),
            surface_container_low: colors.get("surface_container_low").cloned().unwrap_or(hex("#121212")),
            surface_container: colors.get("surface_container").cloned().unwrap_or(hex("#121212")),
            surface_container_high: colors.get("surface_container_high").cloned().unwrap_or(hex("#121212")),
            surface_container_highest: colors.get("surface_container_highest").cloned().unwrap_or(hex("#121212")),

            outline: colors.get("outline").cloned().unwrap_or(hex("#938F99")),
            outline_variant: colors.get("outline_variant").cloned().unwrap_or(hex("#49454F")),
            inverse_surface: colors.get("inverse_surface").cloned().unwrap_or(hex("#E6E1E5")),
            inverse_on_surface: colors.get("inverse_on_surface").cloned().unwrap_or(hex("#121212")),
            inverse_primary: colors.get("inverse_primary").cloned().unwrap_or(hex("#6750A4")),
            scrim: colors.get("scrim").cloned().unwrap_or(Color::srgba(0., 0., 0., 0.5)),
            shadow: colors.get("shadow").cloned().unwrap_or(Color::srgba(0., 0., 0., 0.2)),

            selected: colors.get("selected").cloned().unwrap_or(hex("#D0BCFF")),
            unselected: colors.get("unselected").cloned().unwrap_or(hex("#CCC2DC")),
        })
    }
}

/// Convenience hex parser
#[inline]
fn hex(hex: &str) -> Color {
    Color::from(bevy::prelude::Srgba::hex(hex).unwrap())
}

/// Loader for UiTheme files
#[derive(TypePath, Default, Debug, Deserialize)]
pub struct UiThemeLoader;

impl AssetLoader for UiThemeLoader {
    type Asset = UiTheme;
    type Settings = ();
    type Error = UiAssetLoadError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let disk: DiskUiTheme = ron::de::from_bytes(&bytes)?;

        disk.into_runtime(load_context)
    }

    fn extensions(&self) -> &[&str] {
        &["uitheme.ron"]
    }
}