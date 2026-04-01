use std::ops::{Index, IndexMut};
use serde::{Deserialize, Serialize};
use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};

use crate::ui::{assets::asseterror::UiAssetLoadError, button::WindowState};
use super::atlasbuttonskin::DiskAtlasImage;

/// Titlebar skin stored on disk
#[derive(TypePath, Debug, Deserialize, Serialize)]
pub struct DiskTitlebarSkin {
    pub atlas: DiskAtlasImage,
    pub mapping: [usize; 7],
    pub buttons: usize,
}

impl DiskTitlebarSkin {
    /// Validate the atlas first
    fn validate(&self) -> Result<(), UiAssetLoadError> {
        self.atlas.validate()?;
        let max = self.atlas.max_index();

        for (pos, &idx) in self.mapping.iter().enumerate() {
            if idx >= max {
                return Err(UiAssetLoadError::InvalidMapping { 
                    origin: self.atlas.image_name.clone(),
                    position: pos, index: idx, max: max - 1 
                });
            }
        }
        Ok(())
    }
}

/// Runtime titlebar skin
#[derive(Debug, Clone)]
pub struct TitlebarSkin {
    pub atlas: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
    pub mapping: [usize; 7],
    pub buttons: usize,
}

impl Index<WindowState> for TitlebarSkin {
    type Output = usize;

    fn index(&self, state: WindowState) -> &Self::Output {
        &self.mapping[state.index()]
    }
}

impl IndexMut<WindowState> for TitlebarSkin {
    fn index_mut(&mut self, state: WindowState) -> &mut Self::Output {
        &mut self.mapping[state.index()]
    }
}

impl Default for TitlebarSkin {
    fn default() -> Self {
        Self {
            atlas: Handle::default(),
            image: Handle::default(),
            mapping: [0; 7],
            buttons: 0,
        }
    }
}

impl TitlebarSkin {
    /// Create a runtime titlebar skin from _disk data
    pub fn from_disk(
        disk: &DiskTitlebarSkin,
        load_context: &mut LoadContext<'_>,
    ) -> Self {
        // Load the image handle
        let image_handle: Handle<Image> = load_context.load(&disk.atlas.image_name);

        // Create the atlas layout using the DiskAtlasImage helper
        let layout = disk.atlas.create_layout();
        let layout_handle = load_context.add_labeled_asset("titlebar_layout".into(), layout);

        Self {
            atlas: layout_handle,
            image: image_handle,
            mapping: disk.mapping,
            buttons: disk.buttons,
        }
    }
}

/// Disk window skin
#[derive(TypePath, Debug, Deserialize, Serialize)]
pub struct DiskWindowSkin {
    pub window_image: String,           // main window image
    pub titlebar: DiskTitlebarSkin,     // titlebar atlas & mapping
    pub default_size: [u32; 2],
    pub default_position: [u32; 2],
    pub default_titlebar_position: [u32; 2],
}

impl DiskWindowSkin {
    /// Validate the titlebar atlas & mapping
    pub fn validate(&self) -> Result<(), UiAssetLoadError> {
        self.titlebar.validate()?;
        Ok(())
    }
}

/// Runtime window skin
#[derive(Asset, TypePath, Debug)]
pub struct WindowSkin {
    pub window_image: Handle<Image>,
    pub titlebar: TitlebarSkin,
    pub default_size: UVec2,
    pub default_position: UVec2,
    pub default_titlebar_position: UVec2,
}

impl WindowSkin {
    pub fn from_disk(
        disk: DiskWindowSkin,
        load_context: &mut LoadContext<'_>,
    ) -> Self {
        let window_image_handle: Handle<Image> = load_context.load(&disk.window_image);
        let titlebar = TitlebarSkin::from_disk(&disk.titlebar, load_context);

        Self {
            window_image: window_image_handle,
            titlebar,
            default_size: UVec2::from_array(disk.default_size),
            default_position: UVec2::from_array(disk.default_position),
            default_titlebar_position: UVec2::from_array(disk.default_titlebar_position),
        }
    }
}

/// Asset loader
#[derive(Default, TypePath)]
pub struct WindowSkinLoader;

//#[async_trait::async_trait]
impl AssetLoader for WindowSkinLoader {
    type Asset = WindowSkin;
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

        let disk: DiskWindowSkin = ron::de::from_bytes(&bytes)?;
        disk.validate()?;

        Ok(WindowSkin::from_disk(disk, load_context))
    }

    fn extensions(&self) -> &[&str] {
        &["winskin.ron"]
    }
}