use std::{ops::{Index, IndexMut}};
use serde::{Deserialize, Serialize};
use bevy::{
    asset::LoadContext,
    prelude::*,
    reflect::TypePath,
};

use crate::ui::button::WINDOW_STATE_COUNT;
use crate::ui::{assets::asseterror::UiAssetLoadError, button::UiWindowState};

/// Titlebar skin stored on disk
#[derive(TypePath, Debug, Deserialize, Serialize)]
pub struct DiskTitlebarSkin {
    pub height: f32,
    pub font_name: String,
    pub font_size: f32,
    pub font_color: [f32; 4],
    // atlas image stems from parent window skin
    pub padding: [f32; 4],
    pub mapping: [usize; WINDOW_STATE_COUNT],
    pub buttons: usize,
}

impl DiskTitlebarSkin {
    /// Convert to runtime
    pub fn into_runtime(
        self,
        load_context: &mut LoadContext<'_>,
    ) -> Result<TitlebarSkin, UiAssetLoadError> {
        let font = load_context.load(&self.font_name);
        let font_size = self.font_size;
        let font_color = Color::srgba(self.font_color[0], self.font_color[1], self.font_color[2], self.font_color[3]);

        Ok(TitlebarSkin {
            height: self.height,
            font,
            font_size,
            font_color,
            padding: convert_padding(self.padding),
            mapping: self.mapping,
            buttons: self.buttons,
        })
    }
}

fn convert_padding(padding: [f32; 4]) -> UiRect {
    UiRect {
        left: px(padding[0]),
        right: px(padding[1]),
        top: px(padding[2]),
        bottom: px(padding[3]),
    }
}

/// Runtime titlebar skin
#[derive(Asset, TypePath, Debug, Clone)]
pub struct TitlebarSkin {
    pub height: f32,
    pub font: Handle<Font>,
    pub font_size: f32,
    pub font_color: Color,
    pub padding: UiRect,
    pub mapping: [usize; WINDOW_STATE_COUNT],
    pub buttons: usize, // bitwise or of UiButtonType
}

impl Index<UiWindowState> for TitlebarSkin {
    type Output = usize;

    fn index(&self, state: UiWindowState) -> &Self::Output {
        &self.mapping[state.index()]
    }
}

impl IndexMut<UiWindowState> for TitlebarSkin {
    fn index_mut(&mut self, state: UiWindowState) -> &mut Self::Output {
        &mut self.mapping[state.index()]
    }
}

impl Default for TitlebarSkin {
    fn default() -> Self {
        Self {
            height: 15.0,
            font: Handle::default(),
            font_size: 12.0,
            font_color: Color::srgb(1.0, 0., 0.),
            padding: convert_padding([0.,0.,0.,0.]),
            mapping: [0; WINDOW_STATE_COUNT],
            buttons: 0,
        }
    }
}
