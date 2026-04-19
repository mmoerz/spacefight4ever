use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;

/// marker for a progressbar
#[derive(Component, Default, Clone, Copy)]
pub struct UiProgressBar;

// /// value for a progressbar and material for a shader
// #[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
// pub struct UiProgressBarMaterial {
//     #[uniform(0)]
//     pub progress: f32, // 0.0 to 1.0
//     #[texture(1)]
//     #[sampler(2)]
//     pub texture: Handle<Image>,
// }

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct UiProgressBarMaterial {
    #[uniform(0)]
    pub data: UiProgressBarUniform,

    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

#[derive(ShaderType, Clone, Debug)]
pub struct UiProgressBarUniform {
    pub progress: f32,
    pub uv_offset: Vec2,
    pub uv_scale: Vec2,
    pub direction: u32,
}

/// actual shader that will render the progress
impl UiMaterial for UiProgressBarMaterial {
    fn fragment_shader() -> ShaderRef {
        //"shaders/progress_bar_uv.wgsl".into()
        "shaders/progress_bar_universal.wgsl".into()
    }
}

impl UiProgressBarMaterial {
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
    pub fn set(&mut self, progress: f32) {
        self.data.progress = progress.clamp(0.0, 1.0);
    }
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

/// handle for easier access to the progress (and material)
#[derive(Component, Debug)]
pub struct UiProgressBarHandle(pub Handle<UiProgressBarMaterial>);

/// orientation of a progressbar
#[derive(Clone, Copy, Debug)]
pub enum UiProgressBarDirection {
    LeftToRight,
    RightToLeft,
    BottomToTop,
    TopToBottom,
}

impl UiProgressBarDirection {
    pub fn as_u32(self) -> u32 {
        match self {
            UiProgressBarDirection::LeftToRight => 0,
            UiProgressBarDirection::RightToLeft => 1,
            UiProgressBarDirection::BottomToTop => 2,
            UiProgressBarDirection::TopToBottom => 3,
        }
    }
}

/// a builder for progressbars
pub struct UiProgressBarBuilder {
    pub progress: f32,
    pub width: f32,
    pub height: f32,
    pub offset: Vec2,
    pub scale: Vec2,
    pub direction: UiProgressBarDirection,
    background_texture: Handle<Image>,
    bar_texture: Handle<Image>,
    //material: Handle<UiProgressBarMaterial>,
}

impl UiProgressBarBuilder {
    pub fn new(
        progress: f32,
        width: f32,
        height: f32,
        background_texture: Handle<Image>,
        bar_texture: Handle<Image>,
    ) -> Self {
        Self {
            progress,
            width,
            height,
            offset: Vec2::ZERO,
            scale: Vec2::ONE,
            direction: UiProgressBarDirection::LeftToRight, // default to horizontal
            background_texture,
            bar_texture,
        }
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_direction(mut self, direction: UiProgressBarDirection) -> Self {
        self.direction = direction;
        self
    }

    /// builds a progressbar with a horizontal layout
    pub fn build(
        self,
        materials: &mut Assets<UiProgressBarMaterial>,
    ) -> impl Bundle {
        // in build for easier testing ?
        // must be unique for each progress bar
        let mut mat = UiProgressBarMaterial::new(self.progress, self.bar_texture);
        mat.set_offset(self.offset);
        mat.set_scale(self.scale);
        mat.set_direction(self.direction);

        let handle = materials.add(mat);

        (
            Name::new("UiProgressBar"),
            UiProgressBar,
            UiProgressBarHandle(handle.clone()),
            Node {
                width: px(self.width),
                height: px(self.height),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            ImageNode {
                image: self.background_texture,
                ..default()
            },
            children![(
                // yeah this must be a different node,
                // because material cannot be put inside an node with image
                Node {
                    width: percent(100.0),
                    height: percent(100.0),
                    ..default()
                },
                MaterialNode(handle), 
                //BackgroundColor(Color::WHITE), //debug
            )]
        )       
    }
}

pub fn spawn_progress_bar(
    progress: f32,
    direction: UiProgressBarDirection,
    width: f32,
    height: f32,
    offset: Vec2,
    scale: Vec2,
    background_texture: Handle<Image>,
    bar_texture: Handle<Image>,
    mut commands: Commands,
    mut materials: ResMut<Assets<UiProgressBarMaterial>>,
) -> Entity {
    commands.spawn(
        UiProgressBarBuilder::new(
            progress, width, height, background_texture, bar_texture)
            .with_offset(offset)
            .with_scale(scale)
            .with_direction(direction)
        .build(&mut materials)
    ).id()
}

pub fn progress_bar_bundle(
    progress: f32,
    direction: UiProgressBarDirection,
    width: f32,
    height: f32,
    offset: Vec2,
    scale: Vec2,
    background_texture: Handle<Image>,
    bar_texture: Handle<Image>,
    materials: &mut Assets<UiProgressBarMaterial>,
) -> impl Bundle {
    UiProgressBarBuilder::new(
        progress, width, height, background_texture, bar_texture)
        .with_offset(offset)
        .with_scale(scale)
        .with_direction(direction)
        .build(materials)
}

pub struct UiProgressBarPlugin;

impl Plugin for UiProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(UiMaterialPlugin::<UiProgressBarMaterial>::default())
            ;
    }
}