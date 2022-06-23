use bevy::prelude::*;

use bevy_egui::egui;
use bevy_egui::EguiContext;
use bevy_inspector_egui::egui::Align2;
use bevy_inspector_egui::egui::CollapsingHeader;

pub(crate) struct UiInspectorPlugin;

#[derive(Default)]
struct UiInspectorState {
    visible: bool,
    show_egui_debug_ui: bool,
}

impl Plugin for UiInspectorPlugin {
    fn build(&self, app: &mut App) {
        //

        info!("Building `{}`...", "UiInspectorPlugin");

        app.init_resource::<UiInspectorState>()
            .init_resource::<bevy_inspector_egui::WorldInspectorParams>()
            .add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new())
            .add_startup_system(configure_from_environment_startup_system)
            .add_system(ui_egui_debug);

        //
    }
}

fn configure_from_environment_startup_system(
    mut ui_state: ResMut<UiInspectorState>,
    mut inspector_state: ResMut<bevy_inspector_egui::WorldInspectorParams>,
) {
    //

    info!("System setup...");

    ui_state.visible = !std::env::var_os("QLAD_INSPECTOR").is_none();
    inspector_state.enabled = ui_state.visible;

    ui_state.show_egui_debug_ui = !std::env::var_os("QLAD_EGUI_DEBUG").is_none();

    //
}

fn ui_egui_debug(mut ui_state: ResMut<UiInspectorState>, mut egui_context: ResMut<EguiContext>) {
    //

    let ctx = egui_context.ctx_mut();

    if !ui_state.show_egui_debug_ui {
        return;
    }

    egui::Window::new("egui debug")
        .vscroll(true)
        .anchor(Align2::RIGHT_TOP, [-20.0, 20.0])
        .open(&mut ui_state.show_egui_debug_ui)
        .show(ctx, |ui| {
            //

            CollapsingHeader::new("Settings")
                .default_open(false)
                .show(ui, |ui| {
                    ctx.settings_ui(ui);
                });

            CollapsingHeader::new("Inspection")
                .default_open(false)
                .show(ui, |ui| {
                    ctx.inspection_ui(ui);
                });

            CollapsingHeader::new("Memory")
                .default_open(false)
                .show(ui, |ui| {
                    ctx.memory_ui(ui);
                });

            //
        });

    //
}
