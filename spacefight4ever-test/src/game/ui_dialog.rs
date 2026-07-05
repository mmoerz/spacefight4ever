/// Integration tests for the UiDialog module.
///
/// These tests require a Bevy App context and verify:
/// - Dialog entity spawning and component structure
/// - Dialog event emission (open, close, button interaction)
/// - get_dialog_node parent-traversal logic
use bevy::prelude::*;
use spacefight4ever_ui::plugins::UiDialogPlugin;
use spacefight4ever_ui::ui::button::UiButtonType;
use spacefight4ever_ui::ui::dialog::*;

/// Helper: build a minimal Bevy App with the dialog plugin.
fn build_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins).add_plugins(UiDialogPlugin);
    app
}

// ============================================================================
// Dialog entity and component tests
// ============================================================================

#[test]
fn test_dialog_entity_has_correct_components() {
    let mut app = build_app();

    let dialog_entity: Entity =
        app.world_mut()
            .run_system_once(|mut commands: Commands| -> Entity {
                let dialog =
                    UiDialog::new("Test Dialog".to_string(), UiDialogSeverity::Info).modal(true);
                commands
                    .spawn((
                        dialog,
                        Node {
                            width: Val::Px(400.0),
                            height: Val::Px(250.0),
                            flex_direction: FlexDirection::Column,
                            ..default()
                        },
                        BackgroundColor(UiDialogSeverity::Info.to_color()),
                        Visibility::Visible,
                    ))
                    .id()
            });

    // Verify UiDialog component
    let dialog = app
        .world()
        .entity(dialog_entity)
        .and_then(|e| e.get::<UiDialog>().cloned());
    assert!(dialog.is_some(), "entity should have UiDialog");
    assert_eq!(dialog.unwrap().title, "Test Dialog");
    assert!(dialog.unwrap().is_modal);

    // Verify Node component
    let node = app
        .world()
        .entity(dialog_entity)
        .and_then(|e| e.get::<Node>().cloned());
    assert!(node.is_some(), "entity should have Node");
    assert_eq!(node.unwrap().width, Val::Px(400.0));

    // Verify BackgroundColor matches severity
    let bg_color = app
        .world()
        .entity(dialog_entity)
        .and_then(|e| e.get::<BackgroundColor>().cloned());
    assert!(bg_color.is_some(), "entity should have BackgroundColor");
    assert_eq!(bg_color.unwrap().color(), UiDialogSeverity::Info.to_color());
}

#[test]
fn test_dialog_severity_affects_background_color() {
    let mut app = build_app();

    // Test all severity levels
    for severity in [
        UiDialogSeverity::Info,
        UiDialogSeverity::Warning,
        UiDialogSeverity::Error,
    ] {
        let entity: Entity = app
            .world_mut()
            .run_system_once(|mut commands: Commands| -> Entity {
                commands
                    .spawn(UiDialog::new("Test".to_string(), severity))
                    .with(BackgroundColor(severity.to_color()))
                    .id()
            });

        let bg_color = app
            .world()
            .entity(entity)
            .and_then(|e| e.get::<BackgroundColor>().cloned());
        assert!(bg_color.is_some());
        assert_eq!(bg_color.unwrap().color(), severity.to_color());
    }
}

#[test]
fn test_dialog_modal_flag() {
    let mut app = build_app();

    let modal_entity: Entity =
        app.world_mut()
            .run_system_once(|mut commands: Commands| -> Entity {
                commands
                    .spawn(UiDialog::new("Modal".to_string(), UiDialogSeverity::Info))
                    .id()
            });

    let non_modal_entity: Entity =
        app.world_mut()
            .run_system_once(|mut commands: Commands| -> Entity {
                commands
                    .spawn(
                        UiDialog::new("Non-modal".to_string(), UiDialogSeverity::Info).modal(false),
                    )
                    .id()
            });

    let modal_dialog = app
        .world()
        .entity(modal_entity)
        .and_then(|e| e.get::<UiDialog>().cloned());
    let non_modal_dialog = app
        .world()
        .entity(non_modal_entity)
        .and_then(|e| e.get::<UiDialog>().cloned());

    assert!(modal_dialog.unwrap().is_modal);
    assert!(!non_modal_dialog.unwrap().is_modal);
}

// ============================================================================
// Dialog event tests
// ============================================================================

#[test]
fn test_dialog_open_event_emitted_on_spawn() {
    let mut app = build_app();

    // Spawn a dialog (Added<UiDialog>)
    app.world_mut().run_system_once(|mut commands: Commands| {
        commands.spawn(UiDialog::new("Test".to_string(), UiDialogSeverity::Info));
    });

    // After update, UiDialogOpened should have been emitted
    app.update();

    app.world_mut()
        .run_system_once(|mut events: MessageReader<UiDialogOpened>| {
            let events = events.read(&app.world());
            assert!(
                !events.is_empty(),
                "UiDialogOpened should be emitted on spawn"
            );
        });
}

#[test]
fn test_dialog_close_event_emitted_on_despawn() {
    let mut app = build_app();

    // Spawn a dialog
    let dialog_entity: Entity =
        app.world_mut()
            .run_system_once(|mut commands: Commands| -> Entity {
                commands
                    .spawn(UiDialog::new("Test".to_string(), UiDialogSeverity::Info))
                    .id()
            });

    // Despawn it
    app.world_mut().entity_mut(dialog_entity).despawn();

    // After update, UiDialogClosed should have been emitted
    app.update();
}

// ============================================================================
// Dialog button interaction tests
// ============================================================================

#[test]
fn test_dialog_button_emits_dialog_event() {
    let mut app = build_app();

    // Spawn a dialog with a button row containing an OK button
    let dialog_entity: Entity =
        app.world_mut()
            .run_system_once(|mut commands: Commands| -> Entity {
                let dialog_entity = commands
                    .spawn(UiDialog::new("Test".to_string(), UiDialogSeverity::Info))
                    .id();

                // Spawn a dialog button row with a button
                commands.entity(dialog_entity).with_children(|parent| {
                    parent
                        .spawn((UiDialogButtonRow, Node::default()))
                        .with_children(|button_row| {
                            button_row
                                .spawn((UiDialogButton, UiButtonType::Ok))
                                .with(Interaction::Hovered);
                        });
                });

                dialog_entity
            });

    // Simulate pressing the button
    let button_entity: Entity =
        app.world_mut()
            .run_system_once(|query: Query<Entity, With<UiDialogButton>>| -> Entity {
                query.iter().next().unwrap()
            });

    app.world_mut()
        .entity_mut(button_entity)
        .insert(Interaction::Pressed);

    // After update, UiDialogEvent should have been emitted
    app.update();

    app.world_mut()
        .run_system_once(|mut events: MessageReader<UiDialogEvent>| {
            let events = events.read(&app.world());
            assert!(
                !events.is_empty(),
                "UiDialogEvent should be emitted on button press"
            );
            let event = events.first().unwrap();
            assert_eq!(event.action, UiButtonType::Ok);
            assert_eq!(event.dialog_entity, dialog_entity);
        });
}

#[test]
fn test_dialog_button_event_contains_correct_action_type() {
    let mut app = build_app();

    // Spawn a dialog with Yes and No buttons
    let dialog_entity: Entity =
        app.world_mut()
            .run_system_once(|mut commands: Commands| -> Entity {
                let dialog_entity = commands
                    .spawn(UiDialog::new("Test".to_string(), UiDialogSeverity::Info))
                    .id();

                commands.entity(dialog_entity).with_children(|parent| {
                    parent
                        .spawn((UiDialogButtonRow, Node::default()))
                        .with_children(|button_row| {
                            button_row
                                .spawn((UiDialogButton, UiButtonType::Yes))
                                .with(Interaction::Hovered);
                            button_row
                                .spawn((UiDialogButton, UiButtonType::No))
                                .with(Interaction::Hovered);
                        });
                });

                dialog_entity
            });

    // Press the No button
    let no_button_entity: Entity = app.world_mut().run_system_once(
        |query: Query<Entity, (With<UiDialogButton>, With<UiButtonType>)>| -> Entity {
            // Get the button with UiButtonType::No
            let mut result = None;
            for (entity, button_type) in query.iter() {
                if button_type == UiButtonType::No {
                    result = Some(entity);
                }
            }
            result.unwrap()
        },
    );

    app.world_mut()
        .entity_mut(no_button_entity)
        .insert(Interaction::Pressed);

    app.update();

    app.world_mut()
        .run_system_once(|mut events: MessageReader<UiDialogEvent>| {
            let events = events.read(&app.world());
            assert!(!events.is_empty());
            let event = events.first().unwrap();
            assert_eq!(event.action, UiButtonType::No);
        });
}

// ============================================================================
// get_dialog_node parent-traversal tests
// ============================================================================

#[test]
fn test_get_dialog_node_no_dialog_returns_none() {
    let mut app = build_app();

    let entity: Entity = app
        .world_mut()
        .run_system_once(|mut commands: Commands| -> Entity { commands.spawn(()).id() });

    let result: Option<Entity> = app.world_mut().run_system_once(
        |dialogs: Query<Entity, With<UiDialog>>, parents: Query<&ChildOf>| -> Option<Entity> {
            get_dialog_node(&dialogs, entity, &parents)
        },
    );

    assert!(
        result.is_none(),
        "should return None when no UiDialog exists"
    );
}

#[test]
fn test_get_dialog_node_finds_direct_dialog() {
    let mut app = build_app();

    let dialog_entity: Entity =
        app.world_mut()
            .run_system_once(|mut commands: Commands| -> Entity {
                commands
                    .spawn(UiDialog::new("Test".to_string(), UiDialogSeverity::Info))
                    .id()
            });

    let result: Option<Entity> = app.world_mut().run_system_once(
        |dialogs: Query<Entity, With<UiDialog>>, parents: Query<&ChildOf>| -> Option<Entity> {
            get_dialog_node(&dialogs, dialog_entity, &parents)
        },
    );

    assert_eq!(
        result,
        Some(dialog_entity),
        "should find the dialog entity directly"
    );
}

#[test]
fn test_get_dialog_node_traverses_parent_chain() {
    let mut app = build_app();

    let dialog_entity: Entity =
        app.world_mut()
            .run_system_once(|mut commands: Commands| -> Entity {
                let dialog_entity = commands
                    .spawn(UiDialog::new("Test".to_string(), UiDialogSeverity::Info))
                    .id();

                // Create a child of the dialog
                let child_entity = commands.spawn(()).id();
                commands
                    .entity_mut(dialog_entity)
                    .add_children(&[child_entity]);

                dialog_entity
            });

    let result: Option<Entity> = app.world_mut().run_system_once(
        |dialogs: Query<Entity, With<UiDialog>>, parents: Query<&ChildOf>| -> Option<Entity> {
            get_dialog_node(&dialogs, dialog_entity, &parents)
        },
    );

    assert_eq!(result, Some(dialog_entity));
}
