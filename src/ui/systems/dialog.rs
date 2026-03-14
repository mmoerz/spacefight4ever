
use bevy::prelude::*;

use crate::ui::messages::*;
use crate::ui::dialog_stack::*;
use crate::ui::dialog_manager::*;

pub fn dialog_button_system(
    mut stack: ResMut<DialogStack>,
    mut interactions: Query<
        (
            &Interaction, 
            &DialogButton,
            &mut BackgroundColor
        ),
        (Changed<Interaction>),
    >,
    mut commands: Commands,
    dialog_query: Query<Entity, With<DialogEntity>>,
    mut results: MessageWriter<DialogResult>,
) {
    for (interaction, button, mut background_color) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::srgb(0.25, 0.25, 0.25).into();
                match button {
                    DialogButton::ConfirmExitYes => {
                        results.write(DialogResult::ConfirmExit(true));
                    }
                    DialogButton::ConfirmExitNo => {
                        results.write(DialogResult::ConfirmExit(false));
                    }
                }

                if let Some(dialog_entity) = dialog_query.iter().last() {
                    //despawn_recursive(&mut commands, dialog_entity, &children_query);
                    // should be sufficient and despawn child components
                    commands.entity(dialog_entity).despawn();
                }

                stack.pop();
            }
            Interaction::Hovered => 
                *background_color = Color::srgb(0.4,0.4,0.4).into(),
            Interaction::None => 
                *background_color = Color::WHITE.into(),
        }
    }
}