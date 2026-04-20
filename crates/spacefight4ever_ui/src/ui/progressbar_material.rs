use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;

use crate::ui::progressbar::UiProgressBarDirection;

/// shader stuff for the progressbar

/// material for a progressbar for the shader
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct UiLinearProgressBarMaterial {
    #[uniform(0)]
    pub data: UiProgressBarUniform,

    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

/// uniform parameter for the shader
#[derive(ShaderType, Clone, Debug)]
pub struct UiProgressBarUniform {
    pub progress: f32,
    pub uv_offset: Vec2,
    pub uv_scale: Vec2,
    pub direction: u32,
}

/// actual shader that will render the progress
impl UiMaterial for UiLinearProgressBarMaterial {
    fn fragment_shader() -> ShaderRef {
        //"shaders/progress_bar_uv.wgsl".into()
        "shaders/progress_bar_universal.wgsl".into()
    }
}

impl UiLinearProgressBarMaterial {
    /// create a new progressbarmaterial
    /// each progressbar needs its own material because it
    /// contains the progressbar status (progress)
    pub fn new(progress: f32, texture: Handle<Image>) -> Self {
        Self {
            data: UiProgressBarUniform {
                progress,
                uv_offset: Vec2::ZERO,
                uv_scale: Vec2::ONE,
                direction: 0,
            },
            texture,
        }
    }
    /// set the progress value (0..1.0)
    pub fn set(&mut self, progress: f32) {
        self.data.progress = progress.clamp(0.0, 1.0);
    }
    /// get the progress value (0..1.0)
    pub fn get(&self) -> f32 {
        self.data.progress
    }
    pub fn set_offset(&mut self, offset: Vec2) {
        self.data.uv_offset = offset;
    }
    pub fn set_scale(&mut self, scale: Vec2) {
        self.data.uv_scale = scale;
    }
    pub fn set_direction(&mut self, dir: UiProgressBarDirection) {
        self.data.direction = dir.as_u32();
    }
    pub fn direction(&self) -> UiProgressBarDirection {
        match self.data.direction {
            1 => UiProgressBarDirection::RightToLeft,
            2 => UiProgressBarDirection::BottomToTop,
            3 => UiProgressBarDirection::TopToBottom,
            _ => UiProgressBarDirection::LeftToRight,
        }
    }
}

pub struct UiProgressBarPlugin;

impl Plugin for UiProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UiMaterialPlugin::<UiLinearProgressBarMaterial>::default())
            ;
    }
}