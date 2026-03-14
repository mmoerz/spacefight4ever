use bevy::prelude::*;


#[derive(Component)]
pub struct ButtonWidget;

/// all buttons in the game should be created using this function to ensure
/// consistent styling and behavior
/// which makes it easier for hover / animations / etc. to be added later on
pub fn spawn_button(
    commands: &mut Commands,
    label: &str,
    font: Handle<Font>,
    color: Color,
    background_color:Color,
) -> Bundle {
    commands
        .spawn( (
            Node {
            width: px(120),
            height: px(45),
            justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BackgroundColor(background_color),
            Interaction::None,
            children![(
                Text::new(label),
                TextFont{
                    font: font,
                    font_size: 30.0,
                    ..default()
                },
                TextColor(color)
            )]
        )).id()
}

