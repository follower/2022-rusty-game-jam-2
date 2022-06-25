use bevy::prelude::*;

mod ui_inspector; // TODO: Relocate this.
mod ui_plugin;

mod level_plugin;

fn log_application_start_system() {
    info!("Application startup...");
}

const APP_TITLE: &str = "Quacks Like a Dog (Rusty Jam #2)";

pub fn start_app() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: APP_TITLE.into(),
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(heron::PhysicsPlugin::default()) // TODO: Move into a game-specifc plugin?
        .add_plugin(ui_plugin::UiPlugin)
        .add_plugin(level_plugin::LevelPlugin)
        .add_startup_system(log_application_start_system)
        .run();
}
