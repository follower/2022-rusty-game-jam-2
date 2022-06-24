#![allow(unused_variables, unused_mut, /*dead_code*/)]

use bevy::prelude::*;

use bevy_egui::egui;
use bevy_egui::EguiContext;

use bevy_inspector_egui::Inspectable;

#[derive(Inspectable)]
struct UiTheme {
    //
    button_font_family_name: String,
    button_font_size: f32,
    button_padding: [f32; 2],
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            button_font_family_name: "".to_string(),
            button_font_size: 36.0,
            button_padding: [16.0, 16.0],
        }
    }
}

impl UiTheme {
    //

    fn button(&self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        //

        ui.spacing_mut().button_padding = self.button_padding.into();

        ui.button(
            egui::RichText::new(label)
                .family(egui::FontFamily::Name(
                    self.button_font_family_name.clone().into(),
                ))
                .size(self.button_font_size),
        )

        //
    }
}

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
            .init_resource::<UiTheme>()
            .init_resource::<UiTitleMenuState>()
            .add_startup_system(ui_title_menu_setup)
            .add_system(ui_title_menu);

        //
    }
}

fn ui_title_menu_setup(
    mut ui_state: ResMut<UiTitleMenuState>,
    mut egui_context: ResMut<EguiContext>,
    mut ui_theme: ResMut<UiTheme>,
) {
    //

    info!("System setup...");

    // via <https://github.com/emilk/egui/blob/2ae93c40ab896f31d9976df0c86242b813205722/examples/custom_font/src/main.rs#L14-L41>
    let mut fonts = egui::FontDefinitions::default();

    //

    fonts.font_data.insert(
        "custom_font_01".to_string(),
        egui::FontData::from_static(include_bytes!("../../assets/fonts/Modak/Modak-Regular.ttf",)),
    );

    fonts
        .families
        .entry(egui::FontFamily::Name("custom_font_01".into()))
        .or_default()
        .insert(0, "custom_font_01".to_string());

    //

    fonts.font_data.insert(
        "custom_font_02".to_string(),
        egui::FontData::from_static(include_bytes!(
            //"../../assets/fonts/Oswald/static/Oswald-Bold.ttf",
            //"../../assets/fonts/Oswald/static/Oswald-SemiBold.ttf",
            //"../../assets/fonts/Oswald/static/Oswald-ExtraLight.ttf",
            //"../../assets/fonts/Oswald/static/Oswald-Light.ttf",
            //"../../assets/fonts/Oswald/static/Oswald-Medium.ttf",
            "../../assets/fonts/Oswald/static/Oswald-Regular.ttf",
        )),
    );

    fonts
        .families
        .entry(egui::FontFamily::Name("custom_font_02".into()))
        .or_default()
        .insert(0, "custom_font_02".to_string());

    //

    fonts.font_data.insert(
        "custom_font_03".to_string(),
        egui::FontData::from_static(include_bytes!(
            //"../../assets/fonts/Oswald/static/Oswald-Bold.ttf",
            //"../../assets/fonts/Oswald/static/Oswald-SemiBold.ttf",
            //"../../assets/fonts/Oswald/static/Oswald-ExtraLight.ttf",
            //"../../assets/fonts/Oswald/static/Oswald-Light.ttf",
            "../../assets/fonts/Oswald/static/Oswald-Medium.ttf",
            //"../../assets/fonts/Oswald/static/Oswald-Regular.ttf",
        )),
    );

    fonts
        .families
        .entry(egui::FontFamily::Name("custom_font_03".into()))
        .or_default()
        .insert(0, "custom_font_03".to_string());

    //

    egui_context.ctx_mut().set_fonts(fonts);

    ui_state.visible = true;

    //

    ui_theme.button_font_family_name = "custom_font_03".to_string();
    ui_theme.button_font_size = 48.0;
    ui_theme.button_padding = [24.0, 12.0];

    //
}

fn ui_title_menu(
    ui_state: ResMut<UiTitleMenuState>,
    mut egui_context: ResMut<EguiContext>,
    mut ui_theme: ResMut<UiTheme>,
) {
    //

    let mut ctx = egui_context.ctx_mut();

    if !ui_state.visible {
        return;
    }

    egui::TopBottomPanel::top("title_menu_top_panel")
        .frame(egui::Frame {
            inner_margin: egui::style::Margin {
                left: 25.0,
                top: 0.0, // The title font has huge space above it, so no need to add even more.
                right: 25.0,
                bottom: 25.0,
            },
            ..default()
        })
        .show(ctx, |ui| {
            //
            ui.vertical_centered(|ui| {
                //

                ui.spacing_mut().item_spacing.y = 0.0;

                ui.label(
                    egui::RichText::new("Quacks")
                        .color(egui::Color32::LIGHT_GRAY)
                        .family(egui::FontFamily::Name("custom_font_01".into()))
                        .size(256.0 * 0.8),
                );
                ui.label(
                    egui::RichText::new("LIKE A DOG")
                        .color(egui::Color32::LIGHT_GRAY)
                        .family(egui::FontFamily::Name("custom_font_01".into()))
                        .size(96.0),
                );
            });
        });

    //

    egui::TopBottomPanel::bottom("title_menu_bottom_panel")
        .frame(egui::Frame {
            inner_margin: egui::style::Margin {
                left: 25.0,
                top: 75.0,
                right: 25.0,
                bottom: 75.0,
            },
            ..default()
        })
        .show(ctx, |ui| {
            //

            //
            // This next section is a hack/workaround for egui's currently inability
            // to center multiple/composite/non-atomic widgets.
            //
            // It works by first rendering the (mostly) identical widgets/code
            // invisibly via `set_visible()`, determining how much horizontal
            // space they consume when rendered, and then inserting half that
            // space horizontally before rendering the widgets "for real" in
            // the section after this.
            //
            // (See previous commit message for more details.)
            //
            // Some related issues/discussions on the topic:
            //
            //   * "improvement to columns, its all about the center #1664"
            //     <https://github.com/emilk/egui/discussions/1664>
            //
            //   * "Investigate a multipass (two-pass) version of egui #843"
            //     <https://github.com/emilk/egui/issues/843>
            //
            // I don't think this approach is likely to be particuularly viable
            // for a general "proper" solution to the underlying issue as I'm
            // literally copy-pasting code around in order to ensure I'm creating
            // identical widgets--and that's probably more challenging to reliably
            // achieve programmatically. :D
            //
            // And, of course, it requires keeping the two sections updated by
            // hand when the UI changes--which I'm bound to forget at an inopportune time.
            //
            // Not to mention, when rendering the widgets invisibly, this approach
            // literally wastes vertical screen real estate--which is perhaps
            // computing's most valuable resource currently. #16:9IsAScam
            //
            // Additionally, for a more generic solution the performance impact
            // needs to be factored into things.
            //
            // But for a functional workaround used in small doses it seems feasible.
            // (And it did seem to work with composite/multi-subwidget widgets such
            // as `Slider` which doesn't have the "atomic" special handling some
            // widgets have IIRC).
            //

            let mut spacer_size = 0.0;

            ui.horizontal(|ui| {
                //

                egui::trace!(ui, format!("{:?}", &ui.available_rect_before_wrap()));

                ui.set_visible(false); //

                ui.spacing_mut().item_spacing = [ui_theme.button_padding[0], 0.0].into();

                let tmp = ui_theme.button_padding[1];
                ui_theme.button_padding[1] = 0.0;

                if ui_theme.button(ui, "PLAY").clicked() {
                }

                if ui_theme.button(ui, "OPTIONS").clicked() {
                }

                if ui_theme.button(ui, "CREDITS").clicked() {
                }

                if ui_theme.button(ui, "QUIT").clicked() {
                }

                egui::trace!(ui, format!("{:?}", &ui.min_size()));
                egui::trace!(ui, format!("{:?}", &ui.available_size()));
                egui::trace!(ui, format!("{:?}", &ui.available_rect_before_wrap()));

                spacer_size = (ui.available_rect_before_wrap().width() / 2.0)
                    + (ui.spacing_mut().item_spacing.x / 2.0); //

                egui::trace!(ui, format!("{:?}", spacer_size));

                ui_theme.button_padding[1] = tmp;
                //
            });

            //

            ui.horizontal(|ui| {
                //

                //
                // TODO: Ensure the section above is updated whenever the UI
                //       created in this section is changed.
                //
                //       Otherwise the result will (probably) not be centered
                //       correctly.
                //

                ui.add_space(spacer_size);

                ui.spacing_mut().item_spacing = ui_theme.button_padding.into();

                if ui_theme.button(ui, "PLAY").clicked() {
                    info!("clicked!")
                }

                if ui_theme.button(ui, "OPTIONS").clicked() {
                    info!("clicked!")
                }

                if ui_theme.button(ui, "CREDITS").clicked() {
                    info!("clicked!")
                }

                if ui_theme.button(ui, "QUIT").clicked() {
                    info!("clicked!")
                }

                //
            });
        });

    //
}
