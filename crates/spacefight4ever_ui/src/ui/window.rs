use bevy::prelude::*;

use crate::ui::titlebar::ui_titlebar_bundle;

use super::button::{UiWindowZCounter, UiWindowState, UiWindowType};
use super::assets::{windowsskin::WindowSkin, theme::UiTheme, atlasbuttonskin::ButtonSkin};

#[derive(Component, Clone)]
pub struct UiAtlasWindow {
    pub window_type: UiWindowType,
    pub state: UiWindowState,
    pub skin: Handle<WindowSkin>,
    pub focused: bool,
    pub normal_size: Vec2,
    pub normal_pos: Vec2,
}

pub struct UiAtlasWindowBuilder {
    pub window_type: UiWindowType,
    pub title: String,
    pub skin: Handle<WindowSkin>,
    pub width: f32,
    pub height: f32,
    pub top: f32,
    pub left: f32,
    pub initial_image_node: ImageNode,
    pub z_index: i32,
}

impl UiAtlasWindowBuilder {
    pub fn new(title: String, skin: Handle<WindowSkin>, width: f32, height: f32, top: f32, left: f32, skins: &Assets<WindowSkin>) -> Self {
        //let image_node = ImageNode::default();
        let window_skin  = skins.get(&skin).unwrap();

        let initial_image_node  = 
            ImageNode::from_atlas_image(
                    window_skin.image.clone(),
                    TextureAtlas {
                        index: window_skin.atlas_index,
                        layout: window_skin.atlas.clone(),
                    },
            )
            .with_mode(NodeImageMode::Sliced(window_skin.atlas_slicer.clone()));

        Self {
            window_type: UiWindowType::Standard,
            title,
            skin,
            width,
            height,
            top,
            left,
            initial_image_node,
            z_index: 0,
        }
    }

    pub fn set_skin(mut self, skin: Handle<WindowSkin>) -> Self {
        self.skin = skin;
        self
    }

    pub fn with_z_index(mut self, z_index: &mut UiWindowZCounter) -> Self {
        self.z_index = z_index.inc();
        self
    }

    pub fn build(
        self,
        theme: &UiTheme,
        button_skins: &Assets<ButtonSkin>,
        window_skins: &Assets<WindowSkin>,
    ) -> impl Bundle {
        (
            UiAtlasWindow {
                window_type: self.window_type,
                state: UiWindowState::Normal,
                skin: self.skin.clone(),
                focused: false,
                normal_size: Vec2::new(self.width, self.height),
                normal_pos: Vec2::new(self.left, self.top),
            },
            Node {
                width: Val::Px(self.width),
                height: Val::Px(self.height),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                position_type: PositionType::Absolute,
                left: Val::Px(self.left),
                top: Val::Px(self.top),
                ..default()
            },
            self.initial_image_node,
            GlobalZIndex(self.z_index),

            Transform::default(),
            GlobalTransform::default(),
            Visibility::Inherited,
            children![(
                ui_titlebar_bundle(
                    self.title,
                    self.skin.clone(),
                    theme,
                    self.window_type,
                    button_skins,
                    window_skins)
            )]
        )
    }
}