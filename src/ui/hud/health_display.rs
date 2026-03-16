use bevy::prelude::*;

use crate::game::combat::health::*;
use crate::game::combat::health_basetypes::HealthLayerType;
use crate::game::player::ship::*;

#[derive(Component)]
pub struct HealthBar;

// #[derive(Message)]
// pub enum HealthChanged {
//     Shield(i32),
//     Armor(i32),
//     Hull(i32),
// }

#[derive(Component)]
pub struct UiArmor {
    step: i32,
}

#[derive(Component)]
pub struct UiShield{
    step: i32
}

#[derive(Component)]
pub struct UiHull{
    step: i32
}

pub fn health_display(
    entity: Entity,
    commands: &mut Commands,
    asset_server: &AssetServer
) {
    // Load image
    let ship_image: Handle<Image> = asset_server.load("ships/Durrr.png");
    let armor_image = asset_server.load("ships/armor1.png");
    let shield_paths = [
        "ships/shield1_1.png",
        "ships/shield1_2.png",
        "ships/shield1_3.png",
    ];

    commands
        .entity(entity)
        .with_children(|topparent| {
            topparent.spawn((
                Name::new("Healthbar"),
                Node {
                    width: px(150),
                    height: percent(100),
                    
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                //BackgroundColor(Color::BLACK),
            ))
            .with_children(|healthbar| {
                healthbar.spawn((
                    Node {
                        width: px(80),
                        height: px(80),
                        position_type: PositionType::Absolute,
                        top: px(25),
                        left: px(35),
                        ..default()
                    },
                    ImageNode::new(ship_image),
                ));
            
                // Shields
                for (i, path) in shield_paths.iter().enumerate() {
                    healthbar.spawn(
                        shield_layer(
                            (*path).to_string(), 49,
                            i as i32 * 7 + 3, (i+1) as i32, asset_server
                        )
                    );
                }
                // Armor
                for i in 0..10 {
                    healthbar.spawn(
                        armor_layer(armor_image.clone(), 5, 50 + i*5, 10 - i)
                    );
                }
            });
                // (armor_layer(armor_image.clone(), 5, 50, 10)),
                // (armor_layer(armor_image.clone(), 5, 55, 9)),
                // (armor_layer(armor_image.clone(), 5, 60, 8)),
                // (armor_layer(armor_image.clone(), 5, 65, 7)),
                // (armor_layer(armor_image.clone(), 5, 70, 6)),
                // (armor_layer(armor_image.clone(), 5, 75, 5)),
                // (armor_layer(armor_image.clone(), 5, 80, 4)),
                // (armor_layer(armor_image.clone(), 5, 85, 3)),
                // (armor_layer(armor_image.clone(), 5, 90, 2)),
                // (armor_layer(armor_image.clone(), 5, 95, 1)),
        });
}

fn shield_layer(path: String, left_margin: i32, top: i32, step: i32, asset_server: &AssetServer) -> impl Bundle {
    let shield =  asset_server.load(path);

    (
        Node {
            width: px(50),
            height: px(10),
            position_type: PositionType::Absolute,
            top: px(top),
            left: px(left_margin),
            ..default()
        },
        ImageNode::new(shield),
        UiShield {
            step,
        }
    )
}

fn armor_layer(image: Handle<Image>, left_margin: i32, top: i32, step: i32) -> impl Bundle {
    (
        Node {
            width: px(15),
            height: px(3),
            position_type: PositionType::Absolute,
            top: px(top),
            left: px(left_margin),
            ..default()
        },
        ImageNode::new(image),
        UiArmor {
            step,
        }
    )
}

pub fn update_health_ui(
    ship_query: Query<&ShipHealth, With<PlayerShip>>,
    mut shields: Query<(&UiShield, &mut Visibility)>,
    mut armors: Query<(&UiArmor, &mut Visibility)>,
    mut hulls: Query<(&UiHull, &mut Visibility)>,
) {
    let health = ship_query.single().unwrap();
    let shield_steps_max = shields.iter().len();
    let armor_steps_max = armors.iter().len();
    let hull_steps_max = hulls.iter().len();
    let shield_step_amount = health.values_max[HealthLayerType::Shield] / shield_steps_max as i32;
    let armor_step_amount = health.values_max[HealthLayerType::Armor] / armor_steps_max as i32;
    let hull_step_amount = health.values_max[HealthLayerType::Hull] / hull_steps_max as i32;

    for (layer, mut vis) in &mut shields {
        *vis = if layer.step * shield_step_amount <= health.values[HealthLayerType::Shield] {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    for (layer, mut vis) in &mut armors {
        *vis = if layer.step * armor_step_amount <= health.values[HealthLayerType::Armor] {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }

    for (layer, mut vis) in &mut hulls {
        *vis = if layer.step * hull_step_amount <= health.values[HealthLayerType::Hull] {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}