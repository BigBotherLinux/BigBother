use crate::{state::InstallerState, theme};
use eframe::egui::{self};

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    let total_height = ui.available_height();

    // Use a top-down layout filling the available space
    ui.allocate_ui_with_layout(
        egui::vec2(ui.available_width(), total_height),
        egui::Layout::top_down(egui::Align::Center),
        |ui| {
            // Header
            ui.label(theme::title_text("Disclaimer"));
            ui.add_space(20.0);
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                // Checkbox 1: Disk formatting
                ui.horizontal(|ui| {
                    ui.checkbox(&mut state.disclaimer_format_accepted, "");
                    ui.label(
                        "I understand that this will format my disk and all data will be lost",
                    );
                });
                // Checkbox 2: Unfree software
                ui.horizontal(|ui| {
                    ui.checkbox(&mut state.disclaimer_unfree_accepted, "");
                    ui.label("I accept that unfree software will be installed on my system");
                });

                // Checkbox 3: Surveillance
                ui.horizontal(|ui| {
                    ui.checkbox(&mut state.disclaimer_surveillance_accepted, "");
                    ui.label("I proceed at my own risk and free will");
                });
            });
        },
    );
}
