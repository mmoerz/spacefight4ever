use bevy::prelude::*;

pub struct HudMovementBuilder {
    pub width: f32,
    pub height: f32,
    image: Handle<Image>,
}

impl HudMovementBuilder {
    pub fn new(
        width: f32,
        height: f32,
        asset_server: AssetServer,
    ) -> Self {
        Self {
            width,
            height,
            image: asset_server.load("textures/speedbar.png")
        }
    }

    pub fn build(
        self,
    ) -> impl Bundle {
        (
            Name::new("HudMovement"),
            Node {
                ..default()
            },
            ImageNode {
                image: self.image,
                ..default()
            },
        )       
    }
}