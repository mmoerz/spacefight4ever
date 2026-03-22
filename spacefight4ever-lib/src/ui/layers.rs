/// This module defines the different layers of the UI, 
/// such as the HUD, menu, dialog, and overlay layers.
/// to ensure predictable Z-ordering and make it easier
/// to manage Ui and prespawn the roots of each layer.

use bevy::prelude::*;

use crate::ui::systems::dialog;

use super::bundle::UiNode;
use super::state::*;

#[derive(Component)]
pub struct UiRoot;

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct MenuRoot;

#[derive(Component)]
pub struct DialogRoot;

#[derive(Component)]
pub struct OverlayRoot;

#[derive(Resource, Clone, Copy)]
pub struct UiLayers {
    pub ui_root: Entity,
    pub hud_root: Entity,
    pub menu_root: Entity,
    pub dialog_root: Entity,
    pub overlay_root: Entity,
}

/// creates a layered node structure for ui
pub fn spawn_ui_roots(
    mut commands: Commands,
    mut next_state: ResMut<NextState<UiInitState>>,
) {
    let mut hud_root: Entity = Entity::PLACEHOLDER;
    let mut menu_root = Entity::PLACEHOLDER;
    let mut dialog_root= Entity::PLACEHOLDER;
    let mut overlay_root = Entity::PLACEHOLDER;

    // Spawn UiRoot first
    let ui_root = commands.spawn((
        UiRoot, UiNode::new(
            percent(100),
            percent(100), Color::WHITE,
            JustifyContent::Stretch,
            AlignItems::Stretch
        ), Name::new("UiRoot"))
    ).with_children(|root| {
        hud_root = root.spawn((
            HudRoot, UiNode::default(), Name::new("HudRoot"))).id();
        menu_root = root.spawn((
            MenuRoot, UiNode::default(), Name::new("MenuRoot"))).id();
        dialog_root = root.spawn((
            DialogRoot, UiNode::default(), Name::new("DialogRoot"))).id();
        overlay_root = root.spawn((
            OverlayRoot, UiNode::default(), Name::new("OverlayRoot"))).id();
    })
    .id();

    // Store all layer roots in a resource
    commands.insert_resource(UiLayers {
        ui_root,
        hud_root,
        menu_root,
        dialog_root,
        overlay_root,
    });
    // enable drawing of ui layers
    next_state.set(UiInitState::Ready)
}

/// spawn the ui camera
pub fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Name::new("Camera2d")));
}