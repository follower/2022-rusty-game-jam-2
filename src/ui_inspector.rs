use bevy::prelude::*;

use bevy_egui::egui;
use bevy_egui::EguiContext;

use bevy_inspector_egui::egui::Align2;
use bevy_inspector_egui::egui::CollapsingHeader;
use bevy_inspector_egui::plugin::InspectorWindows;

use crate::ui_plugin::UiTheme;

pub(crate) struct UiInspectorPlugin;

#[derive(Default)]
struct UiInspectorState {
    visible: bool,
    show_custom_inspector_plugins: bool, // i.e. the ones for the types I add via `add_plugin(InspectorPlugin::<Foo>::new())`.
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
            .add_startup_system(configure_inspector_plugins_startup_system)
            .add_system(ui_egui_debug);

        // See comments in `configure_inspector_plugins_startup_system()` for further
        // TODOs related to `InspectorPlugin`s.
        app.add_plugin(bevy_inspector_egui::InspectorPlugin::<UiTheme>::new());

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

    //
    // TODO: Enable by default and/or provide GUI configuration?
    //
    // Note: These really actually are enabled by default but have manually configured
    //       visibility in `configure_inspector_plugins_startup_system()`.
    //
    ui_state.show_custom_inspector_plugins = !std::env::var_os("QLAD_INSPECTOR_PLUGINS").is_none();

    ui_state.show_egui_debug_ui = !std::env::var_os("QLAD_EGUI_DEBUG").is_none();

    //
}

fn configure_inspector_plugins_startup_system(
    ui_state: ResMut<UiInspectorState>,
    mut inspector_windows: ResMut<InspectorWindows>,
) {
    //

    info!("Configure Inspector Gadgets, err, Plugins...");

    if !ui_state.show_custom_inspector_plugins {
        // TODO: Add a way to show them again via GUI.
        info!("Hiding custom inspector plugin windows.");
    }

    //
    // TODO: Figure out if there's both a more automatic way to add these
    //       and a more automatic way to retrieve all that've been added.
    //
    //       Related links:
    //
    //         * <https://github.com/jakobhellermann/bevy-inspector-egui/blob/f73bd92968bdaf8fb0b6e017768038730942ba34/examples/hide_window.rs#L23-L24>
    //         * <https://docs.rs/bevy-inspector-egui/0.11.0/bevy_inspector_egui/plugin/struct.InspectorWindows.html>
    //         * <https://github.com/jakobhellermann/bevy-inspector-egui/issues/25>
    //         * Re: filtering plugins from plugin groups which might be another approach:
    //           <https://maz.digital/mastering-plugin-loadings-bevy-part-22>
    //
    let mut inspector_window_data = inspector_windows.window_data_mut::<UiTheme>();
    inspector_window_data.visible = ui_state.show_custom_inspector_plugins;

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
