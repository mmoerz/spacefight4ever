use bevy::prelude::*;
use spacefight4ever_ui::{
    prelude::{UiElementSize, UiWindowZCounter, window_bundle, UiTheme},
    ui::assets::{assets::UiResources, atlasbuttonskin::ButtonSkin, windowsskin::WindowSkin},
};

pub fn spawn_ship_equipment_dialog(
    commands: &mut Commands,
    parent: Entity,
    asset_server: &Res<AssetServer>,
    z_index: ResMut<UiWindowZCounter>,
    ui_resources: &Res<UiResources>,
    themes: &Assets<UiTheme>,
    skins: &Assets<ButtonSkin>, // pass skins here
    window_skins: &Assets<WindowSkin>, // pass skins here
) -> Entity {
    let theme = themes.get(&ui_resources.theme_handle).unwrap();
    let font_handle: Handle<Font> =
        asset_server.load("fonts/FiraSans-Bold.ttf");
 
    let texture_handle: Handle<Image> = asset_server.load("textures/window/windows_atlas.png");

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
                        theme,
                        skins,
                        window_skins,
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