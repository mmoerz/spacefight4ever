use bevy::prelude::*;
use crate::ui::button::UiWindowType;
use crate::ui::assets::{
    theme::UiTheme,
    atlasbuttonskin::ButtonSkin,
    windowsskin::WindowSkin,
};
use crate::ui::button::UiWindowState;
use crate::ui::atlasbutton::ui_thematic_button_bundle;
use crate::ui::button::UiButtonType;

/// Component for a titlebar that uses a TitlebarSkin atlas
#[derive(Component, Clone)]
pub struct UiTitleBar {
    pub state: UiWindowState,
    pub skin: Handle<WindowSkin>,
}

impl UiTitleBar {
    pub fn new(skin: Handle<WindowSkin>) -> Self {
        Self {
            state: UiWindowState::Normal,
            skin,
        }
    }

    pub fn set_state(mut self, state: UiWindowState) -> Self {
        self.state = state;
        self
    }
}

/// System to update the titlebar visuals based on the current state
pub fn titlebar_update_system(
    skins: Res<Assets<WindowSkin>>,
    mut query: Query<(&UiTitleBar, &mut ImageNode), Changed<UiTitleBar>>,
) {
    for (titlebar, mut image_node) in &mut query {
        if let Some(skin) = skins.get(&titlebar.skin) {
            if let Some(atlas) = &mut image_node.texture_atlas {
                atlas.layout = skin.atlas.clone();
                atlas.index = skin.atlas_index;
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
    title: String,
    titlebar: UiTitleBar,
    height: f32,
    padding: UiRect,
    image: Handle<Image>,
    texture_atlas: TextureAtlas,
}

impl UiTitleBarBuilder {
    pub fn new(title: String, theme: &UiTheme, window_type: UiWindowType, window_skins: &Assets<WindowSkin>, ) -> Self {
        let help = theme.get_window_skin(window_type).expect("Missing window skin in theme");
        let window_skin = window_skins.get(help).expect("Window skin handle not loaded");

        // Prefill the image and atlas for immediate display
        let image = window_skin.image.clone();
        let texture_atlas = TextureAtlas {
            layout: window_skin.atlas.clone(),
            index: window_skin.titlebar[UiWindowState::Normal], // default state
        };

        Self {
            title,
            titlebar: UiTitleBar::new(help.clone()),
            height: window_skin.titlebar.height,
            padding: window_skin.titlebar.padding,
            image,
            texture_atlas,
        }
    }

    pub fn state(mut self, state: UiWindowState) -> Self {
        self.titlebar.state = state;
        self
    }

    fn button_margin(&self) -> UiRect {
        UiRect {
            left: self.padding.left,
            right: self.padding.right,
            top: Val::ZERO,
            bottom: Val::ZERO,
        }
    }            

    pub fn build_with_theme(
        self,
        theme: &UiTheme,
        window_type: UiWindowType,
        button_skins: &Assets<ButtonSkin>,
        window_skins: &Assets<WindowSkin>
    ) -> impl Bundle {
        let b_margin = self.button_margin();
        let help = theme.get_window_skin(window_type).expect("Missing window skin in theme");
        let window_skin = window_skins.get(help).expect("Window skin handle not loaded");
        
        // TODO: calculate text title size and report that back to the window
        // TODO: add slicer to properly use and align the pattern free parts of the image in the corners for the buttons
        (
            self.titlebar,
            Node {
                width: percent(100.),
                height: px(self.height),
                //margin: self.margin,
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                ..default()
            },
            ImageNode {
                image: self.image,
                texture_atlas: Some(self.texture_atlas),
                ..default()
            },
            Visibility::Inherited,
            Pickable {
                should_block_lower: true,
                is_hoverable: true,
                ..default()
            },
            children![(
            // Menu button
                ui_thematic_button_bundle(UiButtonType::Menu, theme, self.height, b_margin, button_skins),
            ),
            // Title text
            (
                Node {
                    height: Val::Px(self.height),
                    justify_content: JustifyContent::Stretch,
                    align_self: AlignSelf::Stretch,
                    ..default()
                },
                Text::new(self.title),
                TextFont {
                    font: window_skin.titlebar.font.clone(),
                    font_size: window_skin.titlebar.font_size.clone(),
                    ..default()
                },
                TextColor(window_skin.titlebar.font_color.clone()),
                Visibility::Inherited,
            ),
            // Right buttons (minimize, maximize, close)
            (
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    margin: UiRect { left: Val::Auto, ..default() },
                    ..default()
                },
                Visibility::Inherited,
                children![
                    ui_thematic_button_bundle(UiButtonType::Minimize, theme, self.height, b_margin, button_skins),
                    ui_thematic_button_bundle(UiButtonType::Maximize, theme, self.height, b_margin, button_skins),
                    ui_thematic_button_bundle(UiButtonType::Close, theme, self.height, b_margin, button_skins),
                ],
            )
            ],
        )
    }
}

/// Helper for spawning a titlebar
pub fn spawn_ui_titlebar(
    commands: &mut Commands,
    title: String,
    theme: &UiTheme,
    window_type: UiWindowType,
    button_skins: &Assets<ButtonSkin>,
    window_skins: &Assets<WindowSkin>
) -> Entity {
    commands.spawn(
        UiTitleBarBuilder::new(title, theme, window_type, window_skins)
            .build_with_theme(theme, window_type, button_skins, window_skins)
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

pub fn ui_titlebar_bundle(
    title: String,
    theme: &UiTheme,
    window_type: UiWindowType,
    button_skins: &Assets<ButtonSkin>,
    window_skins: &Assets<WindowSkin>, // <- pass assets
) -> impl Bundle {
    UiTitleBarBuilder::new(title, theme, window_type, window_skins)
            .build_with_theme(theme, window_type, button_skins, window_skins)
}