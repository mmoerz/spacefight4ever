use bevy::prelude::*;
use super::component::*;

#[derive(Default, Bundle)]
pub struct UiWindowBundle {
    pub window: UiWindow,
    pub state: UiWindowState,

    pub node: Node,
    pub background: BackgroundColor,
    pub image_node: ImageNode,

    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub z_index: GlobalZIndex,
}

#[derive(Bundle)]
pub struct UiButtonBundle {
    pub button: Button,
    pub node: Node,
    pub background: BackgroundColor,
    pub interaction: Interaction,
}

#[derive(Default, Bundle)]
pub struct UiImageButtonBundle {
    pub state: UiImageButtonState,
    pub button: Button,
    pub node: Node,
    pub interaction: Interaction,
}

/// A wrapper for UI text.
#[derive(Bundle)]
pub struct UiTextBundle {
    pub text: Text,
    pub font: TextFont,
    pub color: TextColor,
}

impl UiTextBundle {
    pub fn new(value: &str, font_handle: Handle<Font>, font_size: f32, color: Color) -> Self {
        Self {
            text: Text::new(value.to_string()),
            font: TextFont {
                font: font_handle.clone(),
                font_size: font_size,
                ..default()
            },
            color: TextColor(color),
        }
    }
}

