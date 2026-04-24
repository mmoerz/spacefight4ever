use bevy::prelude::*;
use avian3d::prelude::*;

use crate::game::player::playership::PlayerShip;
use crate::game::ship::modules::propulsion::PropulsionStat;
//use crate::game::player::ship::SpaceshipController;
use crate::ui::input::ship::SpaceshipController;
use crate::game::ship::definitions::ship_definition::{ShipDefinition, ShipDefinitionIndex, ShipModel};

#[derive(Component)]
pub struct HudMovementBar;

use spacefight4ever_ui::ui::{
    progressbar::{UiLinearProgressBar, UiProgressBarDirection, progress_bar_bundle},
    progressbar_material::UiLinearProgressBarMaterial,
    progressbar_commands::UiProgressBarApi,
};

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
        materials: &mut Assets<UiLinearProgressBarMaterial>,
    ) -> impl Bundle {(
            HudMovementBar,
            progress_bar_bundle(0.7, UiProgressBarDirection::LeftToRight,
                120., 16.,
                Vec2 { x: 0.185, y: 0.15 }, Vec2 { x: 1.275, y: 2.1 },
                self.image, self.image_bar, materials)
        )
    }
}

pub fn spawn_movement_bar(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut materials: ResMut<Assets<UiLinearProgressBarMaterial>>,
) -> Entity {
    commands.spawn(
        HudMovementBuilder::new(120.0, 16.0, asset_server)
        .build(&mut materials)
    ).id()
}

pub fn ui_movement_bar_system(
    mut barapi: UiProgressBarApi,
    mut query: Query<Forces, (With<SpaceshipController>, With<PlayerShip>)>,
    ship: Single<&SpaceshipController, (With<SpaceshipController>, With<PlayerShip>)>,
    ship_model: Single<&ShipModel, (With<SpaceshipController>, With<PlayerShip>)>,
    ship_propulsion: Single<&PropulsionStat, (With<SpaceshipController>, With<PlayerShip>)>,
    entity: Single<Entity, With<HudMovementBar>>,
    index: Res<ShipDefinitionIndex>,
    defs: Res<Assets<ShipDefinition>>,
) {
    //let Some(handle) = index.index.get(*ship_model) else { return; };
    //let Some(def) = defs.get(handle) else { return; };
    let max_speed = ship_propulsion.calculate_speed_max(*ship_model, &index, &defs);

    for force in &mut query {
        let controller =  &ship;
        let force_len = force.linear_velocity().length();
        let value = force_len * controller.thrust_multiplier / max_speed;
        if value > 0.1 {
            println!("{:?}", value);
        }
        barapi.set_progress(entity.entity(), value); // display speed in %
    }
}


pub struct MovementDisplayPlugin;

impl Plugin for MovementDisplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, ui_movement_bar_system)
            //.add_plugins(UiMaterialPlugin::<UiProgressBarMaterial>::default())
            //.add_systems(Update, ui_movement_bar_system)
            ;
    }
}