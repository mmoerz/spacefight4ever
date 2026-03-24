use bevy::prelude::*;
use super::messages::*;
use super::layers::*;
use super::dialog_stack::*;
use super::dialogs::confirm_exit::*;
use super::dialogs::message::*;
use super::dialogs::ship_equipment::*;

#[derive(Component)]
pub struct DialogEntity;

#[derive(Component)]
pub enum DialogButton {
    ConfirmExitYes,
    ConfirmExitNo,
}

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

pub fn exit_on_confirm(
    mut events: MessageReader<DialogResult>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for result in events.read() {
        if let DialogResult::ConfirmExit(true) = result {
            println!("Exiting app!");
            app_exit.write(AppExit::Success);
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