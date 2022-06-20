
use bevy::prelude::*;

const APP_TITLE: &str = "Quacks Like a Dog (Rusty Jam #2)";

pub fn start_app() {

    App::new()
	.insert_resource(WindowDescriptor {
	    title: APP_TITLE.into(),
	    ..default()
	})
	.add_plugins(DefaultPlugins)
	.run();

}
