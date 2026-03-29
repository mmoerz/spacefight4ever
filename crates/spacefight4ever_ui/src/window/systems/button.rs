use bevy::prelude::*;

use crate::resource::*;
use crate::component::*;


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