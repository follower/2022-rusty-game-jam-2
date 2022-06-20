use bevy::prelude::*;

pub(crate) struct UiInspectorPlugin;

#[derive(Default)]
struct UiInspectorState {
    visible: bool,
}

impl Plugin for UiInspectorPlugin {
    fn build(&self, app: &mut App) {
        info!("Building `{}`...", "UiInspectorPlugin");

        app.init_resource::<UiInspectorState>()
            .init_resource::<bevy_inspector_egui::WorldInspectorParams>()
            .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
            .add_startup_system(configure_from_environment_startup_system);
    }
}

fn configure_from_environment_startup_system(
    mut ui_state: ResMut<UiInspectorState>,
    mut inspector_state: ResMut<bevy_inspector_egui::WorldInspectorParams>,
) {
    info!("System setup...");

    ui_state.visible = !std::env::var_os("QLAD_INSPECTOR").is_none();
    inspector_state.enabled = ui_state.visible;
}
