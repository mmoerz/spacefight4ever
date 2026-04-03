use std::ops::{Index, IndexMut};
use serde::{Deserialize, Serialize};
use bevy::{
    asset::LoadContext,
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
    pub fn validate(&self) -> Result<(), UiAssetLoadError> {
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

    /// Convert to runtime
    pub fn into_runtime(
        self,
        load_context: &mut LoadContext<'_>,
    ) -> Result<TitlebarSkin, UiAssetLoadError> {
        self.validate()?; // <-- validation lives here

        let image_handle = self.atlas.load_image(load_context);
        let layout = self.atlas.create_layout();
        let layout_handle = 
            load_context.add_labeled_asset(
                format!("titlebar_layout_{}", self.atlas.image_name), layout
            );

        Ok(TitlebarSkin {
            atlas: layout_handle,
            image: image_handle,
            mapping: self.mapping,
            buttons: self.buttons,
        })
    }
}

/// Runtime titlebar skin
#[derive(Asset, TypePath, Debug, Clone)]
pub struct TitlebarSkin {
    pub atlas: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
    pub mapping: [usize; 7],
    pub buttons: usize, // bitwise or of UiButtonType
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
