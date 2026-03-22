/// this file contains convenience functions for easier bundle handling
use bevy::prelude::*;

#[derive(Bundle)]
pub struct UiNode {
    pub node: Node,
    pub background_color: BackgroundColor,
    pub visibility: Visibility,
}

/// helper function to setup ui nodes, however I don't know if this really
/// is a good replacement for NodeBundle (which was removed)
impl UiNode {
    pub fn new(
        width: Val,
        height: Val,
        color: Color,
        justify_content : JustifyContent,
        align_items : AlignItems) 
    -> (Node, BackgroundColor, Visibility) {
        (
            Node {
                width,
                height,
                justify_content,
                align_items,
                display: Display::Block,
                position_type: PositionType::Absolute,
                left: px(0),
                top: px(0),
                ..Default::default()
            },
            BackgroundColor(color),
            Visibility::Visible,
        )
    }
}

impl Default for UiNode {
    /// Default node: fills parent, transparent, column layout
    /// display::Block, PositionType::Absolute, left:px(0), top:px(0)
    fn default() -> Self {
        Self {
            node: Node {
                width: percent(100.0),
                height: percent(100.0),
                display: Display::Block,
                position_type: PositionType::Absolute,
                left: px(0),
                top: px(0),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::srgba(0.,0.,0., 0.)),
            visibility: Visibility::Visible
        }
    }
}

/// A simple button bundle wrapper, ready for interaction.
#[derive(Bundle)]
pub struct UiButtonBundle {
    pub button: Button,
    pub node: Node,
    pub background_color: bevy::ui::BackgroundColor,
    pub interaction: bevy::ui::Interaction,
    pub visibility: Visibility,

    //pub transform: Transform,
    //pub global_transform: GlobalTransform,
}

impl Default for UiButtonBundle {
    fn default() -> Self {
         Self {
            button: Button,
            node: Node {
                width: px(110.0),
                height: px(40.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            visibility: Visibility::Inherited,
            interaction: Interaction::None,
            //transform: Transform::default(),
            //global_transform: GlobalTransform::default(),
        }
    }
}

impl UiButtonBundle {
    pub fn new(
        width: Val,
        height: Val,
        background_color: Color,        
    ) -> Self
    {
        Self {
            button: Button,
            node: Node {
                width: width,
                height: height,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(background_color),
            visibility: Visibility::Inherited,
            interaction: Interaction::None,
        }
    }
}

