use bevy::prelude::*;

#[derive(Component)]
pub struct HealthBar;

#[derive(Message)]
pub enum HealthChanged {
    Health(i64)
}

pub fn health_bar(asset_server: &AssetServer) -> impl Bundle {
    (
        Name::new("Healthbar"),
        Node {
            width: px(150),
            height: px(150),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children!(

        )
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