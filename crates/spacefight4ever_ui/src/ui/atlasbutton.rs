use bevy::prelude::*;

use super::button::ButtonState;
use super::assets::atlasbuttonskin::{ButtonSkin, ButtonSkinLoader};

/// Ui Atlas Button
/// 
/// is a image based button that utilizes an atlas for
/// different button states
#[derive(Component, Clone)]
pub struct UiAtlasButton {
    pub label: String,
    pub state: ButtonState,
    pub skin: Handle<ButtonSkin>,
}

impl Default for UiAtlasButton {
    fn default() -> Self {
        Self {
            label: String::new(),
            state: ButtonState::Normal,
            skin: Handle::default(),
        }
    }
}

impl UiAtlasButton {
    pub fn new(label: impl Into<String>, skin: Handle<ButtonSkin>) -> Self {
        Self {
            label: label.into(),
            state: ButtonState::Normal,
            skin: skin,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.state = ButtonState::Disabled;
        self
    }

    pub fn enabled(mut self) -> Self {
        self.state = ButtonState::Normal;
        self
    }
}

/// system for responding to button interactions
/// 
pub fn button_interaction_system(
    mut query: Query<(&Interaction, &mut UiAtlasButton), Changed<Interaction>>,
) {
    for (interaction, mut button) in &mut query {
        if button.state == ButtonState::Disabled {
            continue; // ignore interactions
        }

        button.state = match *interaction {
            Interaction::Pressed => ButtonState::Pressed,
            Interaction::Hovered => ButtonState::Hovered,
            Interaction::None => ButtonState::Normal,
        };
    }
}

/// This system updates the button's image based on its current state and skin.
fn button_update_atlas_system(
    skins: Res<Assets<ButtonSkin>>,
    mut query: Query<(&UiAtlasButton, &mut ImageNode), Changed<UiAtlasButton>>,
) {
    for (button, mut image_node) in &mut query {
        if let Some(skin) = skins.get(&button.skin) {
            if let Some(atlas) = &mut image_node.texture_atlas {
                atlas.layout = skin.atlas.clone();
                atlas.index = skin[button.state];
            }
            image_node.image = skin.image.clone();
        }
    }
}

pub struct UiAtlasButtonPlugin;

impl Plugin for UiAtlasButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_asset::<ButtonSkin>()
            .init_asset_loader::<ButtonSkinLoader>()
            .add_systems(Update, (
                button_interaction_system,
                button_update_atlas_system
            ));
    }
}

/// Builder for creating Ui buttons.
/// 
/// ## Example with Bevy 0.18
/// 
/// ```no_run
/// use bevy::prelude::*;
/// use spacefight4ever_ui::prelude::*;
/// 
/// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
///     )
/// ```
pub struct UiAtlasButtonBuilder {
    button: UiAtlasButton,
    size: f32,
    margin: UiRect,
}

impl UiAtlasButtonBuilder {
    /// create a new button builder
    pub fn new(label: impl Into<String>, skin: Handle<ButtonSkin>, size: f32, margin: UiRect) -> Self {
        Self {
            button: UiAtlasButton {
                label: label.into(),
                state: ButtonState::Normal,
                skin: skin,
            },
            size,
            margin,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.button.state = ButtonState::Disabled;
        self
    }

    pub fn enabled(mut self) -> Self {
        self.button.state = ButtonState::Normal;
        self
    }

    /// Builds the button as a Bevy bundle
    pub fn build(self) -> impl Bundle {
        (
            self.button,
            Button,
            Node {
                width: Val::Px(self.size),
                height: Val::Px(self.size),
                margin: self.margin,
                ..default()
            },
            ImageNode::default(), // texture/atlas will be resolved in system
            Visibility::Inherited,
        )
    }
}

/// Helper function to spawn a ui atlas button
///
pub fn spawn_ui_button(
    commands: &mut Commands,
    label: impl Into<String>,
    skin: Handle<ButtonSkin>,
    size: f32,
    margin: UiRect,
) -> Entity {
    commands
        .spawn(            
            UiAtlasButtonBuilder::new(label, skin, size, margin)
                .build()
        )
        .id()
}

/// Spawn a material button using the `children!` macro pattern
/// Returns a bundle for inline use
///
/// This is the recommended approach for Bevy 0.17+.
///
/// ## Example:
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_material_ui::prelude::*;
///
/// fn setup(mut commands: Commands, theme: Res<MaterialTheme>) {
///     commands.spawn((
/// 
pub fn ui_button_bundle(
    label: impl Into<String>,
    skin: Handle<ButtonSkin>,

    size: f32,
    margin: UiRect,
) -> impl Bundle {
    UiAtlasButtonBuilder::new(label, skin, size, margin)
        .build()
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

// TODO: add the traits