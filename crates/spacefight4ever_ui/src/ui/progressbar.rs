use bevy::prelude::*;
use bevy::render::render_resource::*;
use bevy::shader::ShaderRef;

/// marker for a progressbar
#[derive(Component, Default, Clone, Copy)]
pub struct UiProgressBar;

/// value for a progressbar and material for a shader
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct UiProgressBarMaterial {
    #[uniform(0)]
    pub progress: f32, // 0.0 to 1.0
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

/// actual shader that will render the progress
impl UiMaterial for UiProgressBarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/progress_bar.wgsl".into()
    }
}

impl UiProgressBarMaterial {
    pub fn new(progress: f32, texture: Handle<Image>) -> Self {
        Self {
            progress,
            texture,
        }
    }
    pub fn set(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }
    pub fn get(&self) -> f32 {
        self.progress
    }
}

/// handle for easier access to the progress (and material)
#[derive(Component, Debug)]
pub struct UiProgressBarHandle(pub Handle<UiProgressBarMaterial>);

#[derive(Clone, Copy)]
pub enum UiProgressBarOrientation {
    Horizontal,
    Vertical,
}

/// a builder for progressbars
pub struct UiProgressBarBuilder {
    pub progress: f32,
    pub width: f32,
    pub height: f32,
    orientation: UiProgressBarOrientation,
    background_texture: Handle<Image>,
    bar_texture: Handle<Image>,
}

impl UiProgressBarBuilder {
    pub fn new(
        progress: f32,
        width: f32,
        height: f32,
        background_texture: Handle<Image>,
        bar_texture: Handle<Image>,
    ) -> Self {
        // must be unique for each progress bar
        
        Self {
            progress,
            width,
            height,
            orientation: UiProgressBarOrientation::Horizontal, // default to horizontal
            background_texture,
            bar_texture,
        }
    }

    pub fn with_orientation(mut self, orientation: UiProgressBarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// builds a progressbar with a horizontal layout
    pub fn build(
        self,
        materials: &mut Assets<UiProgressBarMaterial>,
    ) -> impl Bundle {
        let material = materials.add(UiProgressBarMaterial {
            progress: self.progress,
            texture: self.bar_texture,
        });

        (
            Name::new("HudMovement"),
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
                UiProgressBar,
                Node {
                    width: px(22.0),
                    height: percent(100.0),
                    ..default()
                },
            ),(
                Node {
                    width: percent(78),
                    height: percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                children![(
                    Node{
                        width: percent(100.0),
                        height: percent(15.0),
                        ..default()
                    },
                ), (
                    Node {
                        width: percent(100.0),
                        height: percent(50.0),
                        ..default()
                    },
                    // ImageNode {
                    //     image: self.image_bar,
                    //     ..default()
                    // },
                    UiProgressBarHandle(material.clone()),
                    MaterialNode(material),
                    //BackgroundColor(Color::WHITE),
                )]    
            )],
        )       
    }
}

pub fn spawn_progress_bar(
    progress: f32,
    background_texture: Handle<Image>,
    bar_texture: Handle<Image>,
    mut commands: Commands,
    mut materials: ResMut<Assets<UiProgressBarMaterial>>,
) -> Entity {
    commands.spawn(
        UiProgressBarBuilder::new(
            progress, 120.0, 16.0, background_texture, bar_texture)
        .build(&mut *materials)
    ).id()
}

pub fn progress_bar_bundle(
    progress: f32,
    background_texture: Handle<Image>,
    bar_texture: Handle<Image>,
    materials: &mut Assets<UiProgressBarMaterial>,
) -> impl Bundle {
    UiProgressBarBuilder::new(
        progress, 120.0, 16.0, background_texture, bar_texture)
        .build(materials)
}

/// helper function to set the progress of a progressbar
pub fn set_progress(
    value: f32,
    progressbar: &UiProgressBarHandle,
    materials: &mut Assets<UiProgressBarMaterial>,
) {
    if let Some(mat) = materials.get_mut(&progressbar.0) {
        mat.set(value);
    }
}

/// helper function to get the progress of a progressbar
pub fn get_progress(
    progressbar: &UiProgressBarHandle,
    materials: &mut Assets<UiProgressBarMaterial>,
) -> Option<f32> {
    materials.get(&progressbar.0).map(|mat| mat.get())
}
