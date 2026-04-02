use bevy::prelude::*;

use crate::ui::button::UiButtonType;
use super::button::ButtonState;
use super::assets::atlasbuttonskin::ButtonSkin;
use super::assets::theme::UiTheme;

/// Ui Atlas Button
/// 
/// is a image based button that utilizes an atlas for
/// different button states
#[derive(Component, Clone)]
pub struct UiAtlasButton {
    pub state: ButtonState,
    pub skin: Handle<ButtonSkin>,
}

impl UiAtlasButton {
    pub fn new(skin: Handle<ButtonSkin>) -> Self {
        Self {
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

    pub fn hovered(mut self) -> Self {
        self.state = ButtonState::Hovered;
        self
    }

    pub fn pressed(mut self) -> Self {
        self.state = ButtonState::Pressed;
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

/// Event-driven button update system
/// only executed when a button skin was modified and
/// needs updating in the UI
pub fn button_update_atlas_on_event(
    mut events: MessageReader<AssetEvent<ButtonSkin>>,
    skins: Res<Assets<ButtonSkin>>,
    mut query: Query<(&UiAtlasButton, &mut ImageNode)>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Modified {id, ..} | AssetEvent::Added{id} => {
                for (button, mut image_node) in &mut query {
                    if &button.skin.id() == id {
                        if let Some(skin) = skins.get(*id) {
                            if let Some(atlas) = &mut image_node.texture_atlas {
                                atlas.layout = skin.atlas.clone();
                                atlas.index = skin[button.state];
                            }
                            image_node.image = skin.image.clone();
                        }
                    }
                }
            }
            _  => {}
        }
    }
}

/// Plugin for registering all UI assets
pub struct UiAtlasButtonPlugin;

impl Plugin for UiAtlasButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            // assets are initialized in the assets plugin
            .add_systems(Update, (
                button_interaction_system,
                button_update_atlas_on_event,
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
    initial_image_node: ImageNode,
}

impl UiAtlasButtonBuilder {
    /// create a new button builder
    pub fn new(skin: Handle<ButtonSkin>, size: f32, margin: UiRect,
        skins: &Assets<ButtonSkin>, // <- pass assets
    ) -> Self {
        let mut image_node = ImageNode::default();

        // if the skin is loaded, prefill the image and atlas
        if let Some(skin) = skins.get(&skin) {
            image_node.image = skin.image.clone();
            image_node.texture_atlas = Some(TextureAtlas {
                layout: skin.atlas.clone(),
                index: skin[ButtonState::Normal],
            });
        }

        Self {
            button: UiAtlasButton { state: ButtonState::Normal, skin },
            size,
            margin,
            initial_image_node: image_node, // new field
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
            self.initial_image_node,
            Visibility::Inherited,
        )
    }
}

/// Helper function to spawn a ui atlas button
///
pub fn spawn_ui_button(
    commands: &mut Commands,
    skin: Handle<ButtonSkin>,
    size: f32,
    margin: UiRect,
    skins: &Assets<ButtonSkin>, // <- pass assets
) -> Entity {
    commands.spawn(
        UiAtlasButtonBuilder::new(skin, size, margin, skins)
        .build()
    ).id()
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
    skin: Handle<ButtonSkin>,
    size: f32,
    margin: UiRect,
    skins: &Assets<ButtonSkin>, // <- pass assets
) -> impl Bundle {
    UiAtlasButtonBuilder::new(skin, size, margin, skins)
        .build()
}

pub fn ui_thematic_button_bundle(
    button_type: UiButtonType,
    theme: &UiTheme,
    size: f32,
    margin: UiRect,
    skins: &Assets<ButtonSkin>, // <- pass assets
) -> Option<impl Bundle> {
    theme.button_skins.get(&button_type).map(|skin| {
        ui_button_bundle(skin.clone(), size, margin, skins)
    })
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

pub trait ChildButtonSpawner {
    fn spawn_button(
        &mut self,
        skin: Handle<ButtonSkin>,
        size: f32,
        margin: UiRect,
        skins: &Assets<ButtonSkin>,
    ) -> Entity;

    /// Spawn a button from a `UiTheme` and a `UiButtonType`
    fn spawn_thematic_button(
        &mut self,
        button_type: UiButtonType,
        theme: &UiTheme,
        size: f32,
        margin: UiRect,
        skins: &Assets<ButtonSkin>,
    ) -> Option<Entity>;
}

impl ChildButtonSpawner for ChildSpawnerCommands<'_> {
    fn spawn_button(
        &mut self,
        skin: Handle<ButtonSkin>,
        size: f32,
        margin: UiRect,
        skins: &Assets<ButtonSkin>,
    ) -> Entity {
        self.spawn(ui_button_bundle(skin, size, margin, skins)).id()
    }

    fn spawn_thematic_button(
        &mut self,
        button_type: UiButtonType,
        theme: &UiTheme,
        size: f32,
        margin: UiRect,
        skins: &Assets<ButtonSkin>,
    ) -> Option<Entity> {
        theme.button_skins.get(&button_type).map(|skin| {
            self.spawn_button(skin.clone(), size, margin, skins)
        })
    }
}