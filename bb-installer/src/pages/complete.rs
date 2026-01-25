use crate::{
    state::{InstallStatus, InstallerState},
    theme, widgets,
};
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.add_space(30.0);

        widgets::surveillance_eye(ui, 80.0);

        ui.add_space(20.0);

        ui.label(theme::title_text("Installation Complete"));

        ui.add_space(30.0);

        ui.label("Your system has been successfully installed, reboot to begin using it.");

        ui.add_space(20.0);

        let progress = state.install_progress.lock().unwrap();
        if progress.status == InstallStatus::Failed {
            if let Some(ref error) = progress.error_message {
                widgets::warning_banner(ui, &format!("Installation failed: {}", error));
            }
        }
        drop(progress);

        ui.add_space(30.0);

        if !state.preview_mode {
            if theme::accent_button(ui, "Begin Surveillance (Reboot)").clicked() {
                let _ = std::process::Command::new("reboot").spawn();
            }
        } else {
            ui.label(theme::muted_text("(Preview mode - reboot disabled)"));

            ui.add_space(20.0);

            if theme::secondary_button(ui, "Exit Installer").clicked() {
                std::process::exit(0);
            }
        }

        ui.add_space(30.0);

        ui.label(theme::muted_text(
            "\"Embrace the gaze of BigBother and let it rest upon you.\"",
        ));
    });
}
