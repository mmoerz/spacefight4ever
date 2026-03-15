use bevy::prelude::*;

#[derive(Component)]
pub struct HealthBar;

#[derive(Message)]
pub enum HealthChanged {
    Health(i64)
}

#[derive(Component)]
struct UiArmor {
    step: i32,
}

#[derive(Component)]
struct UiShield{
    step: i32
}

pub fn health_display(asset_server: &AssetServer) -> impl Bundle {
    // Load image
    let ship_image: Handle<Image> = asset_server.load("ships/Durrr.png");
    let armor_image = asset_server.load("ships/armor1.png");
    let left_margin: i32 = 49;

    (
        Name::new("Healthbar"),
        Node {
            width: px(150),
            height: percent(100),
            
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        //BackgroundColor(Color::BLACK),
        children!(
        
            Node {
                width: px(80),
                height: px(80),
                position_type: PositionType::Absolute,
                top: px(25),
                left: px(35),
                ..default()
            },
            ImageNode::new(ship_image),
            
            (shield_layer("ships/shield1_1.png".to_string(), left_margin, 17, 1, asset_server)),
            (shield_layer("ships/shield1_2.png".to_string(), left_margin, 10, 2, asset_server)),
            (shield_layer("ships/shield1_3.png".to_string(), left_margin, 3, 3, asset_server)),
            (armor_layer(armor_image.clone(), 5, 50, 10)),
            (armor_layer(armor_image.clone(), 5, 55, 9)),
            (armor_layer(armor_image.clone(), 5, 60, 8)),
            (armor_layer(armor_image.clone(), 5, 65, 7)),
            (armor_layer(armor_image.clone(), 5, 70, 6)),
            (armor_layer(armor_image.clone(), 5, 75, 5)),
            (armor_layer(armor_image.clone(), 5, 80, 4)),
            (armor_layer(armor_image.clone(), 5, 85, 3)),
            (armor_layer(armor_image.clone(), 5, 90, 2)),
            (armor_layer(armor_image.clone(), 5, 95, 1)),
        )
    )
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
            step: step,
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
            step: step,
        }
    )
}

pub fn health_system(
    mut messages: MessageReader<HealthChanged>,
    mut query: Query<&mut Text, With<HealthBar>>,
) {
    for msg in messages.read() {
        match msg {
            HealthChanged::Health(health) => {
                let mut text = query.single_mut().unwrap();
                *text = Text::new(format!("Health: {}", health))
            }
        }
    }
}