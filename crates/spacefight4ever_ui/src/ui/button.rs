use bevy::prelude::*;

/// Ui Atlas Button
/// 
/// is a image based button that utilizes an atlas for
/// different button states
#[derive(Component, Clone)]
pub struct UiAtlasButton {
    pub label: String,
    pub disabled: bool,
    pub index: usize, // index into the atlas
}

impl UiAtlasButton {
    pub fn new(label: impl Into<String>, index: usize) -> Self {
        Self {
            label: label.into(),
            disabled: false,
            index: index,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub fn index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }
}

pub fn window_button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &UiAtlasButtonIndex, &mut ImageNode),
        (Changed<Interaction>, Or<(
            With<UiWindowMenuButton>,
            With<UiWindowMinimizeButton>,
            With<UiWindowMaximizeButton>,
            With<UiWindowCloseButton>
        )>)>,
    texture_atlases: Res<UiWindowAtlas>,
){
    for (interaction, button_index, mut image_node  ) in &mut interaction_query {
        if let Some(atlas) = &mut image_node.texture_atlas {
            atlas.index = match *interaction {
                Interaction::Pressed => button_index.0 + texture_atlases.button_offset * 2,
                Interaction::Hovered => button_index.0 + texture_atlases.button_offset,
                Interaction::None => button_index.0,
            };
        }
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
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

impl UiAtlasButtonBuilder {
    /// create a new button builder
    pub fn new(
        label: impl Into<String>,
        index: usize,
        size: f32,
        margin: UiRect,
        texture: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
    ) -> Self {
        Self {
            button: UiAtlasButton::new(label, index),
            size,
            margin,
            texture,
            layout,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.button.disabled = true;
        self
    }

    /// Builds the button as a Bevy bundle
    pub fn build(self) -> impl Bundle {
        (
            self.button.clone(),
            Button,
            Node {
                width: Val::Px(self.size),
                height: Val::Px(self.size),
                margin: self.margin,
                ..default()
            },
            ImageNode::from_atlas_image(
                self.texture,
                TextureAtlas {
                    index: self.button.index,
                    layout: self.layout,
                },
            ),
            Visibility::Inherited,
        )
    }
}

/// Helper function to spawn a ui atlas button
///
pub fn spawn_ui_button(
    commands: &mut Commands,
    label: impl Into<String>,
    index: usize,
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    size: f32,
    margin: UiRect,
) -> Entity {
    commands
        .spawn(            
            UiAtlasButtonBuilder::new(label, index, size, margin, texture, layout)
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
    index: usize,
    texture: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
    size: f32,
    margin: UiRect,
) -> impl Bundle {
    UiAtlasButtonBuilder::new(label, index, size, margin, texture, layout)
        .build()
}

// ============================================================================
// Spawn Traits for ChildSpawnerCommands
// ============================================================================

// TODO: add the traits