use bevy::prelude::*;
use super::messages::*;
use super::layers::*;
use super::dialog_stack::*;
use super::dialogs::confirm_exit::*;
use super::dialogs::message::*;

#[derive(Component)]
pub struct DialogEntity;

pub fn dialog_request_system(
    mut requests: MessageReader<DialogRequest>,
    mut stack: ResMut<DialogStack>,
) {
    for req in requests.read() {
        match req {
            DialogRequest::ConfirmExit => {
                stack.push(DialogType::ConfirmExit);
            }
            DialogRequest::Message(msg) => {
                stack.push(DialogType::Message(msg.clone()));
            }
        }
    }
}

// Todo: autoblocking input to lower dialogs
pub fn dialog_spawn_system(
    mut commands: Commands,
    stack: Res<DialogStack>,
    dialog_query: Query<Entity, With<DialogEntity>>,
    ui_layers: Res<UiLayers>,
    asset_server: Res<AssetServer>,
) {
    let existing = dialog_query.iter().count();
    let desired = stack.len();

    if desired <= existing {
        return;
    }

    if let Some(dialog) = stack.top() {
        match dialog {
            DialogType::ConfirmExit => {
                spawn_confirm_exit_dialog(
                    &mut commands,
                    ui_layers.dialog_root,
                    &asset_server);
            }
            DialogType::Message(msg) => {
                spawn_message_dialog(
                    &mut commands,
                    ui_layers.dialog_root,
                    &asset_server,
                    msg);
            }
        }
    }
}

pub fn dialog_button_system(
    mut interactions: Query<
        (&Interaction, &DialogButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut commands: Commands,
    mut stack: ResMut<DialogStack>,
    dialog_query: Query<Entity, With<DialogEntity>>,
    mut results: MessageWriter<DialogResult>,
) {
    for (interaction, button) in &mut interactions {
        if *interaction == Interaction::Pressed {
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
    }
}

// fn despawn_recursive(
//     commands: &mut Commands,
//     entity: Entity, 
//     children_query: &Query<&Children>
// ) {
//     if let Ok(children) = children_query.get(entity) {
//         for child in children.iter() {
//             // recurse into grandchildren
//             despawn_recursive(commands, child, children_query);
//         }
//     }
//     commands.entity(entity).despawn();
// }

#[derive(Component)]
pub enum DialogButton {
    ConfirmExitYes,
    ConfirmExitNo,
}