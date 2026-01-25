use crate::{
    state::{InstallStatus, InstallerState, Page},
    theme::{self, ACCENT_RED, TEXT_MUTED, TEXT_PRIMARY, TEXT_SECONDARY},
    widgets,
};
use eframe::egui::{self, RichText, ScrollArea};

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.label(theme::title_text("Installation Monitor"));
        ui.add_space(5.0);
        ui.label(theme::muted_text("BigBother is being deployed. Please remain calm."));
    });

    ui.add_space(20.0);

    let progress = state.install_progress.lock().unwrap().clone();

    ui.label(RichText::new(progress.status.message()).size(18.0));

    ui.add_space(15.0);

    widgets::progress_bar_surveillance(ui, progress.status.progress(), true);

    ui.add_space(20.0);

    widgets::section_header(ui, "Installation Log");

    ScrollArea::vertical()
        .max_height(250.0)
        .stick_to_bottom(true)
        .show(ui, |ui| {
            for line in &progress.output_log {
                let color = if line.starts_with("ERROR") {
                    ACCENT_RED
                } else if line.starts_with("$") {
                    TEXT_SECONDARY
                } else if line.starts_with("===") {
                    TEXT_PRIMARY
                } else {
                    TEXT_MUTED
                };

                ui.label(RichText::new(line).monospace().color(color));
            }
        });

    if progress.status == InstallStatus::Complete {
        state.current_page = Page::Complete;
    }

    if progress.status == InstallStatus::Failed {
        ui.add_space(10.0);
        if let Some(ref error) = progress.error_message {
            widgets::warning_banner(ui, error);
        }
    }

    ui.ctx().request_repaint();
}
