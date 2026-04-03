use bevy::prelude::*;
use crate::ui::assets::theme::UiTheme;
use crate::ui::assets::titlebarskin::TitlebarSkin;
use crate::ui::button::WindowState;

/// Component for a titlebar that uses a TitlebarSkin atlas
#[derive(Component, Clone)]
pub struct UiTitleBar {
    pub state: WindowState,
    pub skin: Handle<TitlebarSkin>,
}

impl UiTitleBar {
    pub fn new(skin: Handle<TitlebarSkin>) -> Self {
        Self {
            state: WindowState::Normal,
            skin,
        }
    }

    pub fn set_state(mut self, state: WindowState) -> Self {
        self.state = state;
        self
    }
}

/// System to update the titlebar visuals based on the current state
pub fn titlebar_update_system(
    skins: Res<Assets<TitlebarSkin>>,
    mut query: Query<(&UiTitleBar, &mut ImageNode), Changed<UiTitleBar>>,
) {
    for (titlebar, mut image_node) in &mut query {
        if let Some(skin) = skins.get(&titlebar.skin) {
            if let Some(atlas) = &mut image_node.texture_atlas {
                atlas.layout = skin.atlas.clone();
                atlas.index = skin[titlebar.state];
            }
            image_node.image = skin.image.clone();
        }
    }
}

/// Plugin for registering all UI assets
pub struct UiTitleBarPlugin;

impl Plugin for UiTitleBarPlugin {
    fn build(&self, app: &mut App) {
        app
            // assets are initialized in the assets plugin
            .add_systems(Update, 
                titlebar_update_system
            );
    }
}

/// Builder for creating a titlebar
pub struct UiTitleBarBuilder {
    titlebar: UiTitleBar,
    width: f32,
    height: f32,
    margin: UiRect,
    initial_image_node: ImageNode,
}

impl UiTitleBarBuilder {
    pub fn new(skin: Handle<TitlebarSkin>, width: f32, height: f32, margin: UiRect, skins: &Assets<TitlebarSkin>) -> Self {
        let mut image_node = ImageNode::default();

        if let Some(skin) = skins.get(&skin) {
            // Prefill the image and atlas for immediate display
            image_node.image = skin.image.clone();
            image_node.texture_atlas = Some(TextureAtlas {
                layout: skin.atlas.clone(),
                index: skin[WindowState::Normal], // default state
            });
        }

        Self {
            titlebar: UiTitleBar::new(skin),
            width,
            height,
            margin,
            initial_image_node: image_node,
        }
    }

    pub fn state(mut self, state: WindowState) -> Self {
        self.titlebar.state = state;
        self
    }

    pub fn build(self) -> impl Bundle {
        (
            self.titlebar,
            Node {
                width: Val::Px(self.width),
                height: Val::Px(self.height),
                //margin: self.margin,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            self.initial_image_node,
            Visibility::Inherited,
            Pickable {
                should_block_lower: true,
                is_hoverable: true,
                ..default()
            },
            children![(

            )],
        )
    }
}

/// Helper for spawning a titlebar
pub fn spawn_ui_titlebar(
    commands: &mut Commands,
    skin: Handle<TitlebarSkin>,
    width: f32,
    height: f32,
    margin: UiRect,
    skins: &Assets<TitlebarSkin>,
) -> Entity {
    commands.spawn(
        UiTitleBarBuilder::new(skin, width, height, margin, skins)
            .build()
    ).id()
}

// Spawn a titlebar using a `UiTheme`
// pub fn ui_thematic_titlebar_bundle(
//     theme: &UiTheme,
//     width: f32,
//     height: f32,
//     margin: UiRect,
//     skins: &Assets<TitlebarSkin>,
// ) -> Option<impl Bundle> {
//     theme.titlebar_skins.get(&theme.default_titlebar).map(|skin| {
//         UiTitleBarBuilder::new(skin.clone(), width, height, margin, skins).build()
//     })
// }