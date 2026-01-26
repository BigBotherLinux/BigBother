use crate::{theme, widgets};
use eframe::egui;

use crate::state::InstallerState;

pub fn render(ui: &mut egui::Ui, _state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.add_space(40.0);

        widgets::surveillance_eye(ui, 100.0);

        ui.add_space(20.0);

        ui.label(theme::title_text("Welcome to BigBother"));

        ui.add_space(10.0);

        ui.label(theme::subtitle_text(
            "Our source is open. Your curtains should be too..",
        ));
    });
}
