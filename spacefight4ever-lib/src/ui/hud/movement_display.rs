use bevy::prelude::*;


// #[derive(Component)]
// pub struct UiProgressBarValue(pub f32);

pub struct HudMovementBuilder {
    pub width: f32,
    pub height: f32,
    image: Handle<Image>,
    material: Handle<ProgressBarMaterial>,
    image_bar: Handle<Image>,
}

impl HudMovementBuilder {
    pub fn new(
        width: f32,
        height: f32,
        asset_server: &Res<AssetServer>,
        mut materials: ResMut<Assets<ProgressBarMaterial>>,
    ) -> Self {
            let image = asset_server.load("textures/speedbar.png");
            let image_bar = asset_server.load("textures/speedbar_triangles.png");


        // must be unique for each progress bar
        let material = materials.add(ProgressBarMaterial {
            progress: 0.5,
            texture: asset_server.load("textures/speedbar_triangles.png"),
        });
        Self {
            width,
            height,
            material,
        }
    }

    pub fn build(
        self,
    ) -> impl Bundle {
        (
            
        )       
    }
}

pub fn spawn_movement_bar(
    mut commands: Commands,
    asset_server: &Res<AssetServer>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
) -> Entity {
    commands.spawn(
        HudMovementBuilder::new(120.0, 16.0, asset_server, materials)
        .build()
    ).id()
}

pub fn movement_bar_bundle(
    asset_server: &Res<AssetServer>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
) -> impl Bundle {
    HudMovementBuilder::new(120.0, 16.0, asset_server, materials)
        .build()
}

pub fn ui_movement_bar_system(
    mut commands: Commands,
    progressbar_query: Query<Entity, With<UiProgressBarValue>>,
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
            .add_plugins(UiMaterialPlugin::<ProgressBarMaterial>::default())
            //.add_systems(Update, ui_movement_bar_system)
            ;
    }
}