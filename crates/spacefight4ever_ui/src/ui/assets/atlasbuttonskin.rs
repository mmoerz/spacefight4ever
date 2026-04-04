use std::ops::{Index, IndexMut};
use serde::{Deserialize, Serialize};
use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};

use crate::ui::{button::{ButtonState, UiButtonType}};
use super::asseterror::UiAssetLoadError;

/// Intermediate struct for loading from disk
#[derive(Debug, Deserialize, Serialize)]
pub struct DiskAtlasImage {
    pub image_name: String, // path to the texture atlas
    pub tile_size: UVec2,
    pub rows: u32,
    pub cols: u32,
    pub padding: UVec2,
    pub offset: UVec2,
}

impl DiskAtlasImage {
    /// Verify that the atlas is valid
    pub fn validate(&self) -> Result<(), UiAssetLoadError> {
        if self.rows == 0 || self.cols == 0 {
            return Err(UiAssetLoadError::InvalidAtlasSize
                { 
                    origin: self.image_name.clone(),
                    rows: self.rows, cols: self.cols 
                }
            );
        }
        if self.tile_size[0] == 0 || self.tile_size[1] == 0 {
            return Err(UiAssetLoadError::InvalidTileSize
                { 
                    origin: self.image_name.clone(),
                    width: self.tile_size[0], height: self.tile_size[1] 
                }
            );
        }
        Ok(())
    }

    /// Returns the maximum valid index for this atlas
    pub fn max_index(&self) -> usize {
        (self.rows * self.cols) as usize
    }

    /// load the image from disk (get handle)
    #[inline]
    pub fn load_image(&self, load_context: &mut LoadContext<'_>) -> Handle<Image> {
        load_context.load(&self.image_name)
    }

    /// create a layout from the supplied atlas information
    pub fn create_layout(&self) -> TextureAtlasLayout {
        TextureAtlasLayout::from_grid(
            self.tile_size,
            self.cols,
            self.rows,
            Some(self.padding),
            Some(self.offset),
        )
    }

    
}

/// Intermediate struct for deserializing button skin data from disk
#[derive(TypePath, Debug, Deserialize, Serialize)]
pub struct DiskButtonSkin {
    pub atlas: DiskAtlasImage,
    pub states: [usize; 4], // Normal, Hovered, Pressed, Disabled
}

impl DiskButtonSkin {
    pub fn validate(&self) -> Result<(), UiAssetLoadError> {
        self.atlas.validate()?;
        let max = self.atlas.max_index();
        for (pos, &idx) in self.states.iter().enumerate() {
            if idx >= max {
                return Err(UiAssetLoadError::InvalidMapping {
                    origin: self.atlas.image_name.clone(),
                    position: pos,
                    index: idx,
                    max: max - 1,
                });
            }
        }
        Ok(())
    }

    /// 
    pub fn into_runtime(
        self,
        load_context: &mut LoadContext<'_>,
    ) -> Result<ButtonSkin, UiAssetLoadError> {
        self.validate()?; // <-- validation lives here

        let image_handle = self.atlas.load_image(load_context);
        let layout = self.atlas.create_layout();
        let layout_handle = 
            load_context.add_labeled_asset(
                format!("button_layout_{}", &self.atlas.image_name), layout
            );

        Ok(ButtonSkin {
            atlas: layout_handle,
            image: image_handle,
            states: self.states,
        })
    }
}

#[derive(Debug, Clone)]
pub struct UiButtonTypesAllHandles {
    pub types: [Handle<ButtonSkin>; 4],
}

impl Index<UiButtonType> for UiButtonTypesAllHandles {
    type Output = Handle<ButtonSkin>;
    fn index(&self, index: UiButtonType) -> &Self::Output {
        &self.types[index.index()]
    }
}

impl IndexMut<UiButtonType> for UiButtonTypesAllHandles {
    fn index_mut(&mut self, index: UiButtonType) -> &mut Self::Output {
        &mut self.types[index.index()]
    }
}

/// Runtime struct representing a button skin, 
/// which includes handles to the texture and atlas layout for
/// different button states.
#[derive(Asset, TypePath, Debug, Clone)]
pub struct ButtonSkin {
    pub atlas: Handle<TextureAtlasLayout>,
    pub image: Handle<Image>,
    pub states: [usize; 4], // Normal, Hovered, Pressed, Disabled
}

impl Index<ButtonState> for ButtonSkin {
    type Output = usize;

    /// Get the atlas index for a given button state
    /// This allows us to easily switch button appearances based on interaction state
    fn index(&self, state: ButtonState) -> &Self::Output {
        &self.states[state.index()]
    }
}

impl IndexMut<ButtonState> for ButtonSkin {
    /// Set the atlas index for a given button state
    fn index_mut(&mut self, state: ButtonState) -> &mut Self::Output {
        &mut self.states[state.index()]
    }
}

/// custom asset loader for ButtonSkin, 
/// which reads from a RON file and loads the associated texture
#[derive(Default, TypePath)]
pub struct ButtonSkinLoader;

/// Implementation of the custom asset loader for `ButtonSkin`
/// This loader reads a RON file that specifies the texture atlas and mapping for button states,
/// and then loads the associated texture as a Bevy asset.
impl AssetLoader for ButtonSkinLoader {
    type Asset = ButtonSkin;
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
        
        let disk: DiskButtonSkin = ron::de::from_bytes(&bytes)?;

        disk.into_runtime(load_context)
    }

    fn extensions(&self) -> &[&str] {
        &["btnskin.ron"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::Handle;

    /// Helper to create a valid DiskAtlasImage
    fn valid_atlas() -> DiskAtlasImage {
        DiskAtlasImage {
            image_name: "test.png".to_string(),
            tile_size: UVec2::new(16, 16),
            rows: 2,
            cols: 2,
            padding: UVec2::ZERO,
            offset: UVec2::ZERO,
        }
    }

    #[test]
    fn atlas_validation_passes_for_valid_data() {
        let atlas = valid_atlas();
        assert!(atlas.validate().is_ok());
        assert_eq!(atlas.max_index(), 4);
    }

    #[test]
    fn atlas_validation_fails_for_zero_rows() {
        let mut atlas = valid_atlas();
        atlas.rows = 0;
        let err = atlas.validate().unwrap_err();
        if let UiAssetLoadError::InvalidAtlasSize { origin, rows, cols } = err {
            assert_eq!(origin, atlas.image_name.to_string());
            assert_eq!(rows, 0);
            assert_eq!(cols, 2);
        } else {
            panic!("Expected InvalidAtlasSize error");
        }
    }

    #[test]
    fn atlas_validation_fails_for_zero_cols() {
        let mut atlas = valid_atlas();
        atlas.cols = 0;
        let err = atlas.validate().unwrap_err();
        if let UiAssetLoadError::InvalidAtlasSize { origin, rows, cols } = err {
            assert_eq!(origin, atlas.image_name.to_string());
            assert_eq!(rows, 2);
            assert_eq!(cols, 0);
        } else {
            panic!("Expected InvalidAtlasSize error");
        }
    }

    #[test]
    fn atlas_validation_fails_for_zero_tile_size() {
        let mut atlas = valid_atlas();
        atlas.tile_size = UVec2::ZERO;
        let err = atlas.validate().unwrap_err();
        if let UiAssetLoadError::InvalidTileSize { origin, width, height } = err {
            assert_eq!(origin, atlas.image_name.to_string());
            assert_eq!(width, 0);
            assert_eq!(height, 0);
        } else {
            panic!("Expected InvalidTileSize error");
        }
    }

    /// Helper to create a valid DiskButtonSkin
    fn valid_disk_button_skin() -> DiskButtonSkin {
        DiskButtonSkin {
            atlas: valid_atlas(),
            states: [0, 1, 2, 3],
        }
    }

    #[test]
    fn button_skin_validation_passes_for_valid_mapping() {
        let disk = valid_disk_button_skin();
        assert!(disk.validate().is_ok());
    }

    #[test]
    fn button_skin_validation_fails_for_out_of_bounds_index() {
        let mut disk = valid_disk_button_skin();
        disk.states[1] = 10; // out of bounds
        let err = disk.validate().unwrap_err();
        if let UiAssetLoadError::InvalidMapping { origin, position, index, max } = err {
            assert_eq!(origin, disk.atlas.image_name.to_string());
            assert_eq!(position, 1);
            assert_eq!(index, 10);
            assert_eq!(max, 3);
        } else {
            panic!("Expected InvalidMapping error");
        }
    }

    #[test]
    fn button_skin_index_and_index_mut_works() {
        // Mock handles
        let skin = ButtonSkin {
            atlas: Handle::default(),
            image: Handle::default(),
            states: [0, 1, 2, 3],
        };

        use crate::ui::button::ButtonState::*;

        // Index works
        assert_eq!(skin[Normal], 0);
        assert_eq!(skin[Hovered], 1);

        // IndexMut works
        let mut skin_mut = skin.clone();
        skin_mut[Pressed] = 42;
        assert_eq!(skin_mut[Pressed], 42);
    }

    #[test]
    fn create_layout_returns_correct_grid() {
        let atlas = valid_atlas();
        let layout = atlas.create_layout();
        // Check tile size, rows, cols
        assert_eq!(layout.size, UVec2::new(16, 16));
        assert_eq!(layout.textures.len(), 4);
        assert_eq!(layout.textures[0].width(), 16);
        assert_eq!(layout.textures[0].height(), 16);
        assert_eq!(layout.textures[1].width(), 16);
        assert_eq!(layout.textures[1].height(), 16);
        assert_eq!(layout.textures[2].width(), 16);
        assert_eq!(layout.textures[2].height(), 16);
        assert_eq!(layout.textures[3].width(), 16);
        assert_eq!(layout.textures[3].height(), 16);
    }
}