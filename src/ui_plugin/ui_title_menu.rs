use bevy::prelude::*;

use bevy_egui::egui;
use bevy_egui::EguiContext;

#[derive(Default)]
struct UiTitleMenuState {
    visible: bool,
}

pub(super) struct UiTitleMenuPlugin;

impl Plugin for UiTitleMenuPlugin {
    fn build(&self, app: &mut App) {
        //

        info!("Building `{}`...", "UiTitleMenuPlugin");

        app //
            .init_resource::<UiTitleMenuState>()
            .add_startup_system(ui_title_menu_setup)
            .add_system(ui_title_menu);

        //
    }
}

fn ui_title_menu_setup(
    mut ui_state: ResMut<UiTitleMenuState>,
    mut egui_context: ResMut<EguiContext>,
) {
    //

    info!("System setup...");

    // via <https://github.com/emilk/egui/blob/2ae93c40ab896f31d9976df0c86242b813205722/examples/custom_font/src/main.rs#L14-L41>
    let mut fonts = egui::FontDefinitions::default();

    //

    fonts.font_data.insert(
        "custom_font_01".to_string(),
        egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/Modak/Modak-Regular.ttf",
        )),
    );

    fonts
        .families
        .entry(egui::FontFamily::Name("custom_font_01".into()))
        .or_default()
        .insert(0, "custom_font_01".to_string());

    //

    egui_context.ctx_mut().set_fonts(fonts);

    ui_state.visible = true;

    //
}

fn ui_title_menu(ui_state: ResMut<UiTitleMenuState>, mut egui_context: ResMut<EguiContext>) {
    //

    let mut ctx = egui_context.ctx_mut();

    if !ui_state.visible {
        return;
    }

    egui::TopBottomPanel::top("title_menu_top_panel")
        .frame(egui::Frame {
            inner_margin: egui::style::Margin {
                left: 25.0,
                top: 75.0,
                right: 25.0,
                bottom: 25.0,
            },
            ..default()
        })
        .show(ctx, |ui| {
            //
            ui.vertical_centered(|ui| {
                ui.heading(
                    egui::RichText::new("Quacks")
                        .color(egui::Color32::LIGHT_GRAY)
                        .family(egui::FontFamily::Name("custom_font_01".into()))
                        .size(256.0 * 0.8),
                );
                ui.heading(
                    egui::RichText::new("LIKE A DOG")
                        .color(egui::Color32::LIGHT_GRAY)
                        .family(egui::FontFamily::Name("custom_font_01".into()))
                        .size(96.0),
                );
            });
        });

    //
}
