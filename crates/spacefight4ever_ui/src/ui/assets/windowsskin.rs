use serde::{Deserialize, Serialize};
use bevy::{
    asset::{io::Reader, AssetLoader, LoadContext},
    prelude::*,
    reflect::TypePath,
};

use crate::ui::{assets::asseterror::UiAssetLoadError};
use super::atlasbuttonskin::DiskAtlasImage;
use super::titlebarskin::{DiskTitlebarSkin, TitlebarSkin};

/// Disk window skin
#[derive(TypePath, Debug, Deserialize, Serialize)]
pub struct DiskWindowSkin {
    pub window_atlas: DiskAtlasImage,           // main window image
    pub window_atlas_index: usize,
    pub slice_border: [Vec2;2],
    pub titlebar: DiskTitlebarSkin,     // titlebar atlas & mapping
    pub default_size: [u32; 2],
    pub min_size: [u32; 2],
    pub default_position: [u32; 2],
    pub default_titlebar_position: [u32; 2],
}

impl DiskWindowSkin {
    /// Convert to runtime
    pub fn into_runtime(
        self,
        load_context: &mut LoadContext<'_>,
    ) -> Result<WindowSkin, UiAssetLoadError> {
        self.window_atlas.validate()?;
        // TODO: validate that window_atlas_index is inside the atlas

        let image_handle = self.window_atlas.load_image(load_context);
        let layout = self.window_atlas.create_layout();
        let layout_handle = 
            load_context.add_labeled_asset(
                format!("window_layout_{}", self.window_atlas.image_name), layout
            );
        let titlebar = self.titlebar.into_runtime(load_context)?;

        Ok(WindowSkin {
            image: image_handle,
            atlas: layout_handle,
            atlas_index: self.window_atlas_index,
            atlas_slicer: TextureSlicer {
                border: BorderRect { 
                    min_inset: self.slice_border[0], 
                    max_inset: self.slice_border[1], 
                },
                ..Default::default()
            },
            titlebar,
            default_size: UVec2::from_array(self.default_size),
            min_size: UVec2::from_array(self.min_size),
            default_pos: UVec2::from_array(self.default_position),
            default_titlebar_position: UVec2::from_array(self.default_titlebar_position),
        })
    }
}

/// Runtime window skin
#[derive(Asset, TypePath, Debug)]
pub struct WindowSkin {
    pub image: Handle<Image>,
    pub atlas: Handle<TextureAtlasLayout>,
    pub atlas_index: usize,
    pub atlas_slicer: TextureSlicer,
    pub titlebar: TitlebarSkin,
    pub default_size: UVec2,
    pub min_size: UVec2,
    pub default_pos: UVec2,
    pub default_titlebar_position: UVec2,
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

        disk.into_runtime(load_context)
    }

    fn extensions(&self) -> &[&str] {
        &["winskin.ron"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::button::{UiWindowState, WINDOW_STATE_COUNT};

    // fn valid_atlas() -> DiskAtlasImage {
    //     DiskAtlasImage {
    //         image_name: "test.png".to_string(),
    //         tile_size: UVec2::new(16, 16),
    //         rows: 3,
    //         cols: 3,
    //         padding: UVec2::ZERO,
    //         offset: UVec2::ZERO,
    //     }
    // }

    fn valid_titlebar() -> DiskTitlebarSkin {
        DiskTitlebarSkin {
            height: 15.0,
            font_name: "font.ttf".to_string(),
            font_size: 12.0,
            font_color: [1.0, 0.0, 0.0, 1.0],
            padding: [0., 0., 0., 0.],
            mapping: [0, 1, 2, 3, 4],
            buttons: 3,
        }
    }

    fn valid_window() -> DiskWindowSkin {
        DiskWindowSkin {
            window_atlas: DiskAtlasImage {
                image_name: "window.png".to_string(),
                tile_size: UVec2::new(
                    16,
                    16,
                ),
                rows: 3,
                cols: 3,
                padding: UVec2::ZERO,
                offset: UVec2::ZERO,
            },
            window_atlas_index: 0,
            slice_border: [Vec2::ZERO; 2],
            titlebar: valid_titlebar(),
            default_size: [100, 50],
            min_size: [50, 50],
            default_position: [10, 20],
            default_titlebar_position: [0, 0],
        }
    }

    // TODO: add window -> titlebar atlas index validation and test
    // #[test]
    // fn titlebar_validation_passes() {
    //     let titlebar = valid_titlebar();
    //     assert!(titlebar.validate().is_ok());
    // }

    // #[test]
    // fn titlebar_validation_fails_mapping() {
    //     let mut titlebar = valid_titlebar();
    //     titlebar.mapping[3] = 99;

    //     let err = titlebar.validate().unwrap_err();
    //     match err {
    //         UiAssetLoadError::InvalidMapping { position, index, max, .. } => {
    //             assert_eq!(position, 3);
    //             assert_eq!(index, 99);
    //             assert_eq!(max, 8);
    //         }
    //         _ => panic!("Expected InvalidMapping"),
    //     }
    // }

    #[test]
    fn titlebar_index_and_index_mut() {
        use crate::ui::button::UiWindowState::*;

        let mut runtime = TitlebarSkin {
            //image: Handle::default(),
            height: 15.0,
            font: Handle::default(),
            font_size: 12.0,
            font_color: Color::srgb(1.0, 0.,0.),
            padding: UiRect { left: px(0), right: px(0), top: px(0), bottom: px(0) },
            mapping: [0,1,2,3,4],
            buttons: 2,
        };

        assert_eq!(runtime[Normal], 0);
        assert_eq!(runtime[UiWindowState::Closed], 1);

        runtime[UiWindowState::Maximized] = 42;
        assert_eq!(runtime[UiWindowState::Maximized], 42);
    }

    #[test]
    fn titlebar_default() {
        let def = TitlebarSkin::default();

        assert_eq!(def.mapping, [0;WINDOW_STATE_COUNT]);
        assert_eq!(def.buttons, 0);
    }

    #[test]
    fn window_struct_fields_are_correct() {
        let disk = valid_window();

        assert_eq!(disk.titlebar.mapping.len(), 7);
        assert_eq!(disk.titlebar.buttons, 3);
    }

    #[test]
    fn window_position_conversion() {
    let disk = valid_window();

    assert_eq!(UVec2::from_array(disk.default_size), UVec2::new(100, 50));
    assert_eq!(UVec2::from_array(disk.default_position), UVec2::new(10, 20));
    assert_eq!(
        UVec2::from_array(disk.default_titlebar_position),
        UVec2::new(0, 0)
    );
}
}