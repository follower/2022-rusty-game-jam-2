use bevy::prelude::*;

pub(crate) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        info!("Building `{}`...", "UiPlugin");

        app.add_plugin(crate::ui_inspector::UiInspectorPlugin);
    }
}
