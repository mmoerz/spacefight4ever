use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
pub struct ShipModuleBar;

#[derive(Message)]
pub enum HealthChanged {
    Health(i64)
}

pub fn ship_module_bars(asset_server: &AssetServer) -> impl Bundle {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(25.0),
        ..shapes::RegularPolygon::default()
    };
    (
        Name::new("ModuleBar"),
        Node {
            width: px(150),
            height: px(150),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        Visibility::Inherited,
        children!(
            Text::new("mbar"),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 20.0,
                ..default()
            },
            ShapeBuilder::with(&shape)
                .fill(DARK_CYAN)
                .stroke((BLACK, 3.0))
                .build()
        )
    )
}

#[derive(Component)]
pub struct HexGrid;

#[derive(Component)]
pub struct HexCell {
    pub row: usize,
    pub col: usize,
}

#[derive(Resource, Clone)]
pub struct HexGridConfig {
    pub rows: usize,
    pub cols: usize,
    pub hex_radius: f32,
}

pub fn setup_hex_grid(
    parent : Entity,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
//    config: Res<HexGridConfig>,
) {
    // Load SVG image
    let hex_image: Handle<Image> = asset_server.load("ui/yel_hex.png");

    // Config
    let config = HexGridConfig {
        rows: 3,
        cols: 10,
        hex_radius: 20.0,
    };

    // this borrows the config forever
    //commands.insert_resource(config.clone());

    let hex_w = config.hex_radius * 2.0;
    let hex_h = (3.0_f32).sqrt() * config.hex_radius;

    // Spawn parent node for the HUD
    commands.entity(parent)
    .with_children(|parent| {
        parent.spawn((
            Name::new("HexGrid"),
            Node {
                //width: px(500.0),
                //width: percent(100.0),
                width: px((config.cols as f32 + 0.5) * hex_w),
                height: percent(100.0),
                //height: px(150.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                //align_items: AlignItems::FlexStart,
                position_type: PositionType::Relative,
                //position_type: PositionType::Absolute,
                //bottom: px(0),
                ..default()
            },
            HexGrid,
            //BackgroundColor(Color::BLACK),
            //BorderColor::all(BLACK),
        ))
        .with_children(|parent| {
            // debug stuff for checking size (height)
            // parent.spawn((
            //     Node {
            //         width: px(size),
            //         height: px(size),
            //         ..default()
            //     },
            //     BackgroundColor(DARK_RED.into())
            // ));

            for row in 0..config.rows {
                parent.spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        position_type: PositionType::Relative,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                ))
                .with_children(|row_node|{
                    if row  % 2  == 1 {
                        row_node.spawn(Node {
                            width: px(hex_w * 0.5),
                            ..default()
                        });
                    }
                    for col in 0..config.cols {
                        // Offset every other row
                        // let x_offset = if row % 2 == 0 { 0.0 } else { hex_w * 0.5 };
                        // let x = col as f32 * hex_w + x_offset;
                        // let y = row as f32 * hex_h * 1.1; // vertical spacing
                        row_node.spawn(
                            hex_image_button(
                                hex_image.clone(),
                                hex_w,
                                hex_h,
                                row,
                                col)
                        );
                    }
                });
            }
        });
    });
}


#[derive(Component)]
pub struct HexRow;

/// Returns a single hex node with an image child
pub fn hex_image_button(
    hex_image: Handle<Image>,
    hex_width: f32,
    height: f32,
    row: usize,
    col: usize
) -> impl Bundle {
    (
        Name::new("HexButton"),
        Node {
            width: px(hex_width),
            height: px(height),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Relative,
            ..default()
        },
        Visibility::Visible,
        HexCell {
            row,
            col
        },
        ImageNode::new(hex_image),
        Button
    )
}

pub fn ship_module_button_system(
    mut query: Query<(&Interaction, &HexCell), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, hex) in &mut query {
        match *interaction {
            Interaction::Pressed => {
                println!("Hex clicked at row {}, col {}", hex.row, hex.col);
            }
            Interaction::Hovered => {
                // optional: highlight the hex
            }
            Interaction::None => {
                // optional: remove highlight
            }
        }
    }
}

// fn hex_click_system(
//     mouse_input: Input<MouseButton>,
//     windows: Res<Window>,
//     q_hexes: Query<(&GlobalTransform, &Hex)>,
// ) {
//     if mouse_input.just_pressed(MouseButton::Left) {
//         let window = windows.get_primary().unwrap();
//         if let Some(pos) = window.cursor_position() {
//             for (transform, hex) in &q_hexes {
//                 let translation = transform.translation();
//                 // simple bounding box check
//                 let size = Vec2::new(50.0, 50.0); // or hex radius * sqrt(3)
//                 if pos.x >= translation.x
//                     && pos.x <= translation.x + size.x
//                     && pos.y >= translation.y
//                     && pos.y <= translation.y + size.y
//                 {
//                     println!("Clicked hex at row {}, col {}", hex.row, hex.col);
//                 }
//             }
//         }
//     }
// }
