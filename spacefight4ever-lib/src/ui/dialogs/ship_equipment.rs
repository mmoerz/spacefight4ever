use bevy::prelude::*;
use spacefight4ever_ui::prelude::{UiElementSize, UiWindowAtlas, UiWindowZCounter, window_bundle};

pub fn spawn_ship_equipment_dialog(
    commands: &mut Commands,
    parent: Entity,
    asset_server: &Res<AssetServer>,
    mut z_index: ResMut<UiWindowZCounter>,
    window_atlas: &Res<UiWindowAtlas>,
) -> Entity {
    let font_handle: Handle<Font> =
        asset_server.load("fonts/FiraSans-Bold.ttf");
 
    let texture_handle: Handle<Image> = asset_server.load("textures/window/window_sheet.png");
    let buttons_handle: Handle<Image> = asset_server.load("textures/window/button_sheet.png");

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
                    window_bundle(
                        "Ship Equipment",
                        100., 100.,
                        600., 400.,
                        UiElementSize::Small,
                        font_handle.clone(),
                        z_index,
                        texture_handle.clone(),
                        buttons_handle.clone(),
                        window_atlas.window_layout.clone(),
                        window_atlas.button_layout.clone()
                    ),
                    )
                ).id();
        });


    //let window_texture = asset_server.load("textures/slice_square_2.png");

    // commands
    //     .entity(parent)
    //     .with_children(|parent| {
    //         window_id =
    //             parent.spawn(
    //                 window_ninepatch::create_ui_ninepatch_window(
    //                     px(100.),
    //                     px(100.),
    //                     px(600.),
    //                     px(400.),
    //                     window_texture
    //             )
    //         ).id();
    //     });

    window_id
}