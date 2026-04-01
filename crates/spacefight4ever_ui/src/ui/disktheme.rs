use std::collections::HashMap;
use ron::de;
use thiserror::Error;
use serde::{Deserialize, Serialize};
use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};

use crate::ui::assets::atlasbuttonskin::{ButtonSkin, DiskButtonSkin};

use super::theme::*;
use super::assets::windowsskin::*;

#[derive(TypePath, Debug, Deserialize, Serialize)]
pub struct DiskUiTheme {
    // Button skins keyed by name
    pub button_skins: HashMap<String, DiskButtonSkin>,
    
    // Window skins keyed by name
    pub window_skins: HashMap<String, DiskWindowSkin>,

    // Optional global colors
    pub colors: HashMap<String, [f32; 4]>, // RGBA
}

#[derive(TypePath, Debug, Deserialize)]
pub struct UiThemeLoader;

#[derive(Debug, Error)]
pub enum UiThemeLoaderError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("RON Error: {0}")]
    Ron(#[from] ron::error::SpannedError),
}

impl AssetLoader for UiThemeLoader {
    type Asset = UiTheme;
    type Settings = ();
    type Error = UiThemeLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let disk: DiskUiTheme = ron::de::from_bytes(&bytes)?;

        let mut button_skins = HashMap::new();
        for (name, disk_button) in disk.button_skins {
            let image_handle = load_context.load(&disk_button.image_name);
            let layout = TextureAtlasLayout::from_grid(
                UVec2::new(disk_button.tile_size[0], disk_button.tile_size[1]),
                disk_button.atlas.cols,
                disk_button.atlas.rows,
                None,
                None,
            );
            let layout_handle = load_context.add_labeled_asset(format!("button_layout_{}", name), layout);
            button_skins.insert(
                name,
                load_context.set_labeled_asset_handle(format!("button_skin_{}", name), ButtonSkin {
                    atlas: layout_handle,
                    image: image_handle,
                    mapping: disk_button.mapping,
                }),
            );
        }

        let mut window_skins = HashMap::new();
        for (name, disk_window) in disk.window_skins {
            let window_image = load_context.load(&disk_window.image_name);
            let titlebar_image = load_context.load(&disk_window.titlebar_image_name);
            let layout = TextureAtlasLayout::from_grid(
                UVec2::new(disk_window.tile_size[0], disk_window.tile_size[1]),
                disk_window.cols,
                disk_window.rows,
                None,
                None,
            );
            let atlas_handle = load_context.add_labeled_asset(format!("titlebar_layout_{}", name), layout);
            let titlebar = TitlebarSkin {
                atlas: atlas_handle,
                image: titlebar_image,
                mapping: disk_window.mapping,
                buttons: disk_window.buttons,
            };
            window_skins.insert(
                name,
                load_context.set_labeled_asset_handle(format!("window_skin_{}", name), WindowSkin {
                    window_image,
                    titlebar,
                    default_size: UVec2::from_array(disk_window.default_size),
                    default_position: UVec2::from_array(disk_window.default_position),
                    default_titlebar_position: UVec2::from_array(disk_window.default_titlebar_position),
                }),
            );
        }

        let colors = disk.colors.into_iter()
            .map(|(k, rgba)| (k, Color::rgba(rgba[0], rgba[1], rgba[2], rgba[3])))
            .collect();

        Ok(UiTheme { button_skins, window_skins, colors })
    }

    fn extensions(&self) -> &[&str] {
        &["uitheme.ron"]
    }
}