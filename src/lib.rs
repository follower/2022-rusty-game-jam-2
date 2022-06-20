use bevy::prelude::*;

mod ui_plugin;

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
        .add_plugins(DefaultPlugins)
        .add_plugin(ui_plugin::UiPlugin)
        .add_startup_system(log_application_start_system)
        .run();
}
