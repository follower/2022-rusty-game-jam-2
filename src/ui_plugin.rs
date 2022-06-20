use bevy::prelude::*;

mod ui_title_menu;

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
