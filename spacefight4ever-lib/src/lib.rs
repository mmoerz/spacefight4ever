use bevy::prelude::*;

pub mod config;
pub mod plugin;
pub mod ui;
pub mod game;

pub mod prelude {
    pub use crate::plugin::*;
    pub use crate::ui::*;
    pub use crate::game::*;
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn( (
        Text::new("hello foo"),
        Underline,
        TextFont{
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 67.0,
            ..default()
        },
        TextShadow::default(),
        // Set the justification of the Text
        TextLayout::new_with_justify(Justify::Center),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            bottom: px(5),
            right: px(5),
            ..default()
        },
    ));
}