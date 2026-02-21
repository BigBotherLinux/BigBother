use crate::{
    state::{InstallStatus, InstallerState},
    theme::{self, ACCENT_RED, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY},
    widgets,
};
use eframe::egui::{self, RichText, ScrollArea};

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.add_space(10.0);

        widgets::surveillance_eye(ui, 60.0);

        ui.add_space(10.0);

        ui.label(theme::title_text("Installation Complete"));

        ui.add_space(10.0);

        let progress = state.install_progress.lock().unwrap();

        if progress.status == InstallStatus::Failed {
            if let Some(ref error) = progress.error_message {
                widgets::warning_banner(ui, &format!("Installation failed: {}", error));
            }
        } else if !state.production_mode {
            ui.label(theme::muted_text(
                "Dry-run completed. Commands that would have been executed:",
            ));
        } else {
            ui.label("Your system has been successfully installed, reboot to begin using it.");
        }

        ui.add_space(10.0);

        // Show installation log
        widgets::section_header(ui, "Installation Log");

        ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            for line in &progress.output_log {
                let color = if line.starts_with("ERROR") {
                    ACCENT_RED
                } else if line.starts_with("$") {
                    TEXT_SECONDARY
                } else if line.starts_with("===") || line.starts_with("***") {
                    TEXT_PRIMARY
                } else {
                    TEXT_MUTED
                };
                ui.label(RichText::new(line).monospace().color(color).size(11.0));
            }
        });

        drop(progress);

        ui.add_space(15.0);

        if state.production_mode {
            if theme::accent_button(ui, "Reboot (remember to remove installation media)").clicked() {
                let _ = std::process::Command::new("reboot").spawn();
            }
        } else {
            ui.label(theme::muted_text("(Dry-run mode - no changes were made)"));

            ui.add_space(10.0);

            if theme::secondary_button(ui, "Exit Installer").clicked() {
                std::process::exit(0);
            }
        }

        ui.add_space(10.0);

        ui.label(theme::muted_text(
            "\"Embrace the gaze of BigBother and let it rest upon you.\"",
        ));
    });
}
