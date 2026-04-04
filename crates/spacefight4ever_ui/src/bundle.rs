use bevy::prelude::*;

#[derive(Bundle)]
pub struct  UiButtonBundle {
    pub button: Button,
    pub node: Node,
    pub background: BackgroundColor,
    pub interaction: Interaction,
}

impl UiButtonBundle {
    pub fn new(width: Val, height: Val, color: Color) -> Self {
        Self {
            button: Button,
            node: Node {
                width: width,
                height: height,
                ..default()
            },
            background: BackgroundColor(color),
            interaction: Interaction::None,
        }
    }
}

#[derive(Default, Bundle)]
pub struct UiImageButtonBundle {
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

