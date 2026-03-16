use bevy::prelude::*;

pub fn button_system(
    mut query: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::Pressed => 
                *background_color = Color::srgb(0.25, 0.25, 0.25).into(),
            Interaction::Hovered => 
                *background_color = Color::srgb(0.4,0.4,0.4).into(),
            Interaction::None => 
                *background_color = Color::WHITE.into(),
        }
    }
}