/// Dialog component for the UI system.
///
/// A dialog is a modal window that blocks input to other UI elements when open.
/// It consists of:
/// - A title bar (using UiTitleBar)
/// - A content area (for text or custom UI)
/// - A button bar (for action buttons like OK, Cancel, Yes, No, etc.)
///
/// Usage example:
///
/// ```no_run
/// use bevy::prelude::*;
/// use spacefight4ever_ui::prelude::*;
/// use spacefight4ever_ui::ui::dialog::*;
///
/// fn spawn_confirm_dialog(
///     mut commands: Commands,
///     theme: Res<UiTheme>,
///     button_skins: Res<Assets<ButtonSkin>>,
///     window_skins: Res<Assets<WindowSkin>>,
/// ) {
///     spawn_confirm_dialog_impl(
///         &mut commands,
///         "Exit Game?".to_string(),
///         "Are you sure you want to exit?".to_string(),
///         &theme,
///         &button_skins,
///         &window_skins,
///     );
/// }
/// ```
use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;

use crate::ui::assets::atlasbuttonskin::ButtonSkin;
use crate::ui::assets::theme::UiTheme;
use crate::ui::assets::windowsskin::WindowSkin;
use crate::ui::atlasbutton::ui_thematic_button_bundle;
use crate::ui::button::{UiButtonType, UiWindowType};
use crate::ui::titlebar::ui_titlebar_bundle;

// ============================================================================
// Dialog Severity Levels
// ============================================================================

/// Dialog severity levels for styling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiDialogSeverity {
    Info,
    Warning,
    Error,
}

impl UiDialogSeverity {
    /// Get the background color for this severity
    /// TODO: use colors from theme (maybe theme needs to be extended prior)
    pub fn to_color(&self) -> Color {
        match self {
            UiDialogSeverity::Info => Color::srgb(0.1, 0.1, 0.1),
            UiDialogSeverity::Warning => Color::srgb(0.8, 0.5, 0.1),
            UiDialogSeverity::Error => Color::srgb(0.8, 0.1, 0.1),
        }
    }
}

// ============================================================================
// Dialog Component
// ============================================================================

/// Marker component for the button row in a dialog
#[derive(Component)]
pub struct UiDialogButtonRow;

/// Marker component for dialog buttons
#[derive(Component)]
pub struct UiDialogButton;

/// Component that marks a UI element as a dialog
#[derive(Component, Clone)]
pub struct UiDialog {
    /// The dialog title
    pub title: String,
    /// The severity level (affects styling)
    pub severity: UiDialogSeverity,
    /// Whether the dialog is modal (blocks input to other UI)
    pub is_modal: bool,
}

impl UiDialog {
    /// Create a new dialog with the given title and severity
    pub fn new(title: String, severity: UiDialogSeverity) -> Self {
        Self {
            title,
            severity,
            is_modal: true,
        }
    }

    /// Set whether the dialog is modal
    pub fn modal(mut self, is_modal: bool) -> Self {
        self.is_modal = is_modal;
        self
    }
}

// ============================================================================
// Dialog Events
// ============================================================================

/// Event sent when a dialog button is clicked
#[derive(Message, Debug, Clone)]
pub struct UiDialogEvent {
    /// The button type that was clicked (matches UiButtonType values)
    pub action: UiButtonType,
    /// The entity of the dialog that was responded to
    pub dialog_entity: Entity,
}

/// Event sent when a dialog is opened
#[derive(Message, Debug, Clone)]
pub struct UiDialogOpened {
    pub dialog_entity: Entity,
}

/// Event sent when a dialog is closed
#[derive(Message, Debug, Clone)]
pub struct UiDialogClosed {
    pub dialog_entity: Entity,
}

// ============================================================================
// Dialog Builder
// ============================================================================

/// Builder for creating dialogs with a message and action buttons
pub struct UiDialogBuilder {
    dialog: UiDialog,
    dialog_message: String,
    dialog_buttons: Vec<UiButtonType>,
    window_type: UiWindowType,
    width: f32,
    height: f32,
}

impl UiDialogBuilder {
    /// Create a new dialog builder
    pub fn new(
        title: String,
        message: String,
        actions: Vec<UiButtonType>,
        window_type: UiWindowType,
    ) -> Self {
        Self {
            dialog: UiDialog::new(title, UiDialogSeverity::Info),
            dialog_message: message,
            dialog_buttons: actions,
            window_type,
            width: 400.0,
            height: 250.0,
        }
    }

    /// Set the dialog size
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Set the dialog severity
    pub fn severity(mut self, severity: UiDialogSeverity) -> Self {
        self.dialog.severity = severity;
        self
    }

    /// Set the message text
    pub fn message(mut self, message: String) -> Self {
        self.dialog_message = message;
        self
    }

    /// Set the action buttons
    pub fn actions(mut self, actions: Vec<UiButtonType>) -> Self {
        self.dialog_buttons = actions;
        self
    }

    /// Build the dialog and return its entity
    pub fn build(
        self,
        commands: &mut Commands,
        theme: &UiTheme,
        button_skins: &Assets<ButtonSkin>,
        window_skins: &Assets<WindowSkin>,
    ) -> Entity {
        // Spawn the dialog root
        let dialog_entity = commands
            .spawn((
                self.dialog.clone(),
                Node {
                    width: px(self.width),
                    height: px(self.height),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Stretch,
                    margin: UiRect::all(Val::Auto),
                    ..default()
                },
                BackgroundColor(self.dialog.severity.to_color()),
                Visibility::Visible,
            ))
            .id();

        // Add children: title bar, message, and button row
        commands.entity(dialog_entity).with_children(|parent| {
            // Add title bar (includes menu, minimize, maximize, close buttons)
            parent.spawn(ui_titlebar_bundle(
                self.dialog.title.clone(),
                theme,
                self.window_type,
                button_skins,
                window_skins,
            ));

            // Add message text in the center of the content area
            parent.spawn((
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(px(20.0)),
                    ..default()
                },
                Text::new(self.dialog_message.clone()),
                TextFont {
                    font: Handle::default(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor::WHITE,
                Visibility::Inherited,
            ));

            // Add action buttons row at the bottom
            parent
                .spawn((
                    UiDialogButtonRow,
                    Node {
                        width: Val::Percent(100.),
                        height: px(40.0),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::FlexEnd,
                        column_gap: px(10.0),
                        padding: UiRect::all(px(10.0)),
                        ..default()
                    },
                    Visibility::Inherited,
                ))
                .with_children(|button_row| {
                    for &button_type in &self.dialog_buttons {
                        let button_bundle = ui_thematic_button_bundle(
                            button_type,
                            theme,
                            30.0,
                            UiRect::all(px(5.0)),
                            button_skins,
                        );
                        button_row.spawn((
                            UiDialogButton,
                            button_type,
                            button_bundle,
                            Visibility::Inherited,
                        ));
                    }
                });
        });

        dialog_entity
    }
}

// ============================================================================
// Dialog Spawn Functions
// ============================================================================

/// Spawn a dialog with a message and custom action buttons
pub fn spawn_ui_dialog(
    commands: &mut Commands,
    title: String,
    message: String,
    actions: Vec<UiButtonType>,
    window_type: UiWindowType,
    theme: &UiTheme,
    button_skins: &Assets<ButtonSkin>,
    window_skins: &Assets<WindowSkin>,
) -> Entity {
    UiDialogBuilder::new(title, message, actions, window_type).build(
        commands,
        theme,
        button_skins,
        window_skins,
    )
}

/// Spawn a confirmation dialog (Yes/No buttons)
pub fn spawn_confirm_dialog(
    commands: &mut Commands,
    title: String,
    message: String,
    theme: &UiTheme,
    button_skins: &Assets<ButtonSkin>,
    window_skins: &Assets<WindowSkin>,
) -> Entity {
    spawn_ui_dialog(
        commands,
        title,
        message,
        vec![UiButtonType::Yes, UiButtonType::No],
        UiWindowType::Standard,
        theme,
        button_skins,
        window_skins,
    )
}

/// Spawn an OK/Cancel dialog
pub fn spawn_ok_cancel_dialog(
    commands: &mut Commands,
    title: String,
    message: String,
    theme: &UiTheme,
    button_skins: &Assets<ButtonSkin>,
    window_skins: &Assets<WindowSkin>,
) -> Entity {
    spawn_ui_dialog(
        commands,
        title,
        message,
        vec![UiButtonType::Ok, UiButtonType::Cancel],
        UiWindowType::Standard,
        theme,
        button_skins,
        window_skins,
    )
}

/// Spawn a simple message dialog (OK button only)
pub fn spawn_message_dialog(
    commands: &mut Commands,
    title: String,
    message: String,
    theme: &UiTheme,
    button_skins: &Assets<ButtonSkin>,
    window_skins: &Assets<WindowSkin>,
) -> Entity {
    spawn_ui_dialog(
        commands,
        title,
        message,
        vec![UiButtonType::Ok],
        UiWindowType::Standard,
        theme,
        button_skins,
        window_skins,
    )
}

// ============================================================================
// Dialog Button Interaction System
// ============================================================================

// helper function to retrieve the parent UiAtlasWindow entity
/// is used when dragging a titlebar, to obtain the parent window
pub fn get_dialog_node(
    windows: &Query<Entity, With<UiDialog>>,
    mut current: Entity,
    parents: &Query<&ChildOf>,
) -> Option<Entity> {
    while let Ok(parent) = parents.get(current) {
        if windows.contains(current) {
            return Some(current);
        }
        current = parent.get();
    }

    windows.contains(current).then_some(current)
}

/// System that handles dialog button interactions.
///
/// When a button with `UiButtonType` component inside a dialog is pressed,
/// this system emits a `UiDialogEvent` message with the button type and
/// the dialog entity.
pub fn dialog_button_interaction_system(
    mut q: Query<
        (Entity, &UiButtonType, &Interaction),
        (Changed<Interaction>, With<UiDialogButton>),
    >,
    parents: Query<&ChildOf>,
    dialogs: Query<Entity, With<UiDialog>>,
    mut events: MessageWriter<UiDialogEvent>,
) {
    // Find all pressed dialog buttons
    for (button_entity, button_type, interaction) in &mut q {
        if *interaction == Interaction::Pressed {
            if let Some(dialog) = get_dialog_node(&dialogs, button_entity, &parents) {
                // Found the parent dialog entity
                events.write(UiDialogEvent {
                    action: *button_type,
                    dialog_entity: (dialog),
                });
            }
        }
    }
}

/// System that emits `UiDialogOpened` event when a dialog is spawned
pub fn dialog_open_system(
    mut events: MessageWriter<UiDialogOpened>,
    query: Query<Entity, Added<UiDialog>>,
) {
    for entity in &query {
        events.write(UiDialogOpened {
            dialog_entity: entity,
        });
    }
}

/// System that emits `UiDialogClosed` event when a dialog is despawned
pub fn dialog_close_system(
    mut events: MessageWriter<UiDialogClosed>,
    query: Query<(Entity, Ref<UiDialog>)>,
) {
    for (entity, refDlg) in &query {
        events.write(UiDialogClosed {
            dialog_entity: entity,
        });
    }
}

// ============================================================================
// Dialog Plugin
// ============================================================================

/// Plugin for the UiDialog system.
///
/// Registers the dialog events and systems for handling button interactions.
pub struct UiDialogPlugin;

impl Plugin for UiDialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<UiDialogEvent>()
            .add_message::<UiDialogOpened>()
            .add_message::<UiDialogClosed>()
            .add_systems(
                Update,
                (
                    dialog_button_interaction_system,
                    dialog_open_system,
                    dialog_close_system,
                ),
            );
    }
}
