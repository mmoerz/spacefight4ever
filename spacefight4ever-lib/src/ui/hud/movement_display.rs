use bevy::prelude::*;
use spacefight4ever_ui::ui::progressbar::{UiProgressBar, UiProgressBarDirection, UiProgressBarMaterial, progress_bar_bundle};

pub struct HudMovementBuilder {
    pub width: f32,
    pub height: f32,
    image: Handle<Image>,
    image_bar: Handle<Image>,
}

impl HudMovementBuilder {
    pub fn new(
        width: f32,
        height: f32,
        asset_server: &Res<AssetServer>,
    ) -> Self {
        let image = asset_server.load("textures/speedbar.png");
        let image_bar = asset_server.load("textures/speedbar_triangles.png");

        Self {
            width,
            height,
            image,
            image_bar
        }
    }

    pub fn build(
        self,
        materials: &mut Assets<UiProgressBarMaterial>,
    ) -> impl Bundle {
        progress_bar_bundle(0.7, UiProgressBarDirection::LeftToRight,
            120., 16.,
            Vec2 { x: 0.185, y: 0.15 }, Vec2 { x: 1.275, y: 2.1 },
            self.image, self.image_bar, materials)
    }
}

pub fn spawn_movement_bar(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut materials: ResMut<Assets<UiProgressBarMaterial>>,
) -> Entity {
    commands.spawn(
        HudMovementBuilder::new(120.0, 16.0, asset_server)
        .build(&mut materials)
    ).id()
}

// pub fn movement_bar_bundle(
//     asset_server: &Res<AssetServer>,
//     mut materials: ResMut<Assets<UiProgressBarMaterial>>,
// ) -> impl Bundle {
//     HudMovementBuilder::new(120.0, 16.0, asset_server)
//         .build(&mut *materials)
// }

pub fn ui_movement_bar_system(
    mut commands: Commands,
    progressbar_query: Query<Entity, With<UiProgressBar>>,
    mut image_query: Query<&mut ImageNode, With<UiProgressBar>>,
) {
    for progessbar in progressbar_query.iter() {
        if let Ok(mut image_node) = image_query.get_mut(progessbar) {

        }
    }    
}

pub struct MovementDisplayPlugin;

impl Plugin for MovementDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            //.add_plugins(UiMaterialPlugin::<UiProgressBarMaterial>::default())
            //.add_systems(Update, ui_movement_bar_system)
            ;
    }
}