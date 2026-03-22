use bevy::prelude::*;
use crate::ui::window::structs::UiElementSize;
use crate::ui::window::window;

/// Spawns a "Confirm Exit" dialog under the DialogRoot layer
pub fn spawn_ship_equipment_dialog(
    commands: &mut Commands,
    parent: Entity,
    asset_server: &Res<AssetServer>,
) -> Entity {
    let font_handle: Handle<Font> =
        asset_server.load("fonts/FiraSans-Bold.ttf");
    let icon_menu: Handle<Image> = 
        asset_server.load("ui/ButtonsSmall [Normal]/Button-4.png");
    let icon_menu_hover: Handle<Image> = 
        asset_server.load("ui/ButtonsSmall [Hover]/Button-4.png");
    let icon_menu_disabled: Handle<Image> = 
        asset_server.load("ui/ButtonsSmall [Disabled]/Button-4.png");

    let icon_close: Handle<Image> = 
        asset_server.load("ui/ButtonsSmall [Normal]/Button-1.png");
    let icon_close_hover: Handle<Image> = 
        asset_server.load("ui/ButtonsSmall [Hover]/Button-1.png");
    let icon_close_disabled: Handle<Image> = 
        asset_server.load("ui/ButtonsSmall [Disabled]/Button-1.png");

    let icon_minimize: Handle<Image> = 
        asset_server.load("ui/ButtonsSmall [Normal]/Button-3.png");
    let icon_minimize_hover: Handle<Image> = 
        asset_server.load("ui/ButtonsSmall [Hover]/Button-3.png");
    let icon_minimize_disabled: Handle<Image> = 
        asset_server.load("ui/ButtonsSmall [Disabled]/Button-3.png");
    
    let icon_maximize: Handle<Image> =
        asset_server.load("ui/ButtonsSmall [Normal]/Button-2.png");
    let icon_maximize_hover: Handle<Image> =
        asset_server.load("ui/ButtonsSmall [Hover]/Button-2.png");
    let icon_maximize_disabled: Handle<Image> =
        asset_server.load("ui/ButtonsSmall [Disabled]/Button-2.png");

    match asset_server.get_load_state(&font_handle) {
        Some(bevy::asset::LoadState::Loaded) => println!("Loaded"),
        Some(bevy::asset::LoadState::Failed(e)) => println!("FAILED: {:?}", e),
        _ => println!("Loading..."),
    }

    let mut window_id = Entity::PLACEHOLDER;

    // Spawn dialog panel under DialogRoot
    commands
        .entity(parent)
        .with_children(|parent| {
            window_id =
                parent.spawn((
                    window::window_bundle(
                        "Ship Equipment",
                        100., 100.,
                        600., 400.,
                        UiElementSize::Small,
                        font_handle.clone(),
                        icon_menu.clone(),
                        icon_menu_hover.clone(),
                        icon_menu_disabled.clone(),
                        icon_close.clone(),
                        icon_close_hover.clone(),
                        icon_close_disabled.clone(),
                        icon_minimize.clone(),
                        icon_minimize_hover.clone(),
                        icon_minimize_disabled.clone(),
                        icon_maximize.clone(),
                        icon_maximize_hover.clone(),
                        icon_maximize_disabled.clone(),
                    ),
                    )
                ).id();
        });

    window_id
}