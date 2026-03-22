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
    let icon_close: Handle<Image> = 
        asset_server.load("ui/ButtonsSmall [Normal]/Button-1.png");
    let icon_minimize: Handle<Image> = 
        asset_server.load("ui/ButtonsSmall [Normal]/Button-3.png");
    let icon_maximize: Handle<Image> =
        asset_server.load("ui/ButtonsSmall [Normal]/Button-2.png");

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
                        icon_close.clone(),
                        icon_minimize.clone(),
                        icon_maximize.clone(),
                    ),
                    )
                ).id();
        });
            //     DialogEntity,
            //     Name::new("Ship_Equiment_Dialog"),
            //     Node {
            //         width: px(600.0),
            //         height: px(400.0),
            //         // flex_direction: FlexDirection::Row,
            //         // justify_content: JustifyContent::Start,
            //         // align_items: AlignItems::,
            //         // position_type: PositionType::Absolute,
            //         align_self: AlignSelf::Auto,
            //         left: px(100.0),
            //         top: px(100.0),
            //         ..default()
            //     },
            //     BackgroundColor(Color::srgba(0.99, 0., 0., 0.9)),
            //     Visibility::Visible,
            
            //     children![
            //         (
            //             Node {
            //                 width: px(15),
            //                 height: percent(100),
            //                 ..default()
            //             },
            //             BackgroundColor(Color::srgb(0., 0.5, 0.5))
            //         ), (
            //             Node {
            //                 width: px(20),
            //                 height: percent(100),
            //                 ..default()
            //             },
            //             BackgroundColor(Color::srgb(0., 0.2, 0.5))
            //         ), (
            //             Node {
            //                 width: px(240),
            //                 height: percent(100),
            //                 ..default()
            //             },
            //             BackgroundColor(Color::srgb(0., 0.5, 0.5))
            //         ), (
            //             Node {
            //                 width: px(20),
            //                 height: percent(100),
            //                 ..default()
            //             },
            //             BackgroundColor(Color::srgb(0., 0.2, 0.5))
            //         ), (
            //             Node {
            //                 width: px(25),
            //                 height: percent(100),
            //                 ..default()
            //             },
            //             BackgroundColor(Color::srgb(0., 0.5, 0.5))
            //         ), (
            //             Node {
            //                 width: px(260),
            //                 height: percent(100),
            //                 ..default()
            //             },
            //             BackgroundColor(Color::srgb(0., 0.2, 0.5))
            //         ), (
            //             Node {
            //                 width: px(20),
            //                 height: percent(100),
            //                 ..default()
            //             },
            //             BackgroundColor(Color::srgb(0., 0.5, 0.5))

            //         )
            //     ]
            // ));
            // .with_children(|dialog| {
            //     dialog.spawn((
            //         Node {
            //             width: px(20),
            //             height: percent(100),
            //             ..default()
            //         },
            //         BackgroundColor(Color::srgb(0., 0.5, 0.5))
            //     ));
            // });
        // })
        // ;

    window_id
}