use bevy::prelude::*;

pub mod plugin;
pub mod ui;
pub mod game;

use plugin::UiPlugin;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiPlugin)

        .add_systems(Startup, setup)
        .add_systems(Update, trigger_exit_dialog)

        //.add_systems(Update, crate::ui::debug::debug_print_ui_tree)
        .run();
}

fn trigger_exit_dialog(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut events: MessageWriter<ui::messages::DialogRequest>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        events.write(ui::messages::DialogRequest::ConfirmExit);
    }
}



fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn( (
        Text::new("hello foo"),
        Underline,
        TextFont{
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 67.0,
            ..default()
        },
        TextShadow::default(),
        // Set the justification of the Text
        TextLayout::new_with_justify(Justify::Center),
        // Set the style of the Node itself.
        Node {
            position_type: PositionType::Absolute,
            bottom: px(5),
            right: px(5),
            ..default()
        },
    ));
}