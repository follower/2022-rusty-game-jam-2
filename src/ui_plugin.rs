use bevy::prelude::*;

use bevy_egui::egui;

mod ui_title_menu;

//
// This possibly isn't the best location for `UiTheme`, it might
// be better in a separate module/file.
//
// But it does make more sense here than where it was in `ui_title_menu`
// previously, and for it to be accessible to other module and/or crate members.
//
#[derive(bevy_inspector_egui::Inspectable)]
pub(crate) struct UiTheme {
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

pub(crate) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        //

        info!("Building `{}`...", "UiPlugin");

        app //
            .add_plugin(bevy_egui::EguiPlugin)
            .add_plugin(crate::ui_inspector::UiInspectorPlugin)
            .add_plugin(ui_title_menu::UiTitleMenuPlugin);

        //
    }
}
