use bevy::prelude::*;

use super::progressbar_material::UiLinearProgressBarMaterial;

/// marker for a progressbar
//#[derive(Component, Default, Clone, Copy)]
// pub trait UiProgressBar : Component {
//     fn progress(&self) -> f32;
//     fn set_progress(&mut self, progress: f32);
// }

/// handle for easier access to the progress (and material)
#[derive(Component, Debug)]
pub struct UiLinearProgressBar(pub Handle<UiLinearProgressBarMaterial>);

/// orientation of a progressbar which defines the fill direction
#[derive(Clone, Copy, Debug)]
pub enum UiProgressBarDirection {
    /// fills the bar left to right
    LeftToRight, 
    /// fills the bar right to left
    RightToLeft, 
    /// fills the bar bottom to top
    BottomToTop, 
    /// fills the bar top to bottom
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
        materials: &mut Assets<UiLinearProgressBarMaterial>,
    ) -> impl Bundle {
        // in build for easier testing ?
        // must be unique for each progress bar
        let mut mat = UiLinearProgressBarMaterial::new(self.progress, self.bar_texture);
        mat.set_offset(self.offset);
        mat.set_scale(self.scale);
        mat.set_direction(self.direction);

        let handle = materials.add(mat);

        (
            Name::new("UiProgressBar"),
            UiLinearProgressBar(handle.clone()),
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
                // because material cannot be put inside a node with image
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
    mut materials: ResMut<Assets<UiLinearProgressBarMaterial>>,
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
    materials: &mut Assets<UiLinearProgressBarMaterial>,
) -> impl Bundle {
    UiProgressBarBuilder::new(
        progress, width, height, background_texture, bar_texture)
        .with_offset(offset)
        .with_scale(scale)
        .with_direction(direction)
        .build(materials)
}

