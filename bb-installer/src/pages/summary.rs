use crate::{state::InstallerState, theme, widgets};
use eframe::egui::{self};

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.label(theme::title_text("Pre-Installation Briefing"));
        ui.add_space(5.0);
        ui.label(theme::muted_text(
            "Review your configuration before submission",
        ));
    });

    ui.add_space(20.0);

    if state.preview_mode {
        widgets::preview_mode_banner(ui);
        ui.add_space(10.0);
    }

    widgets::section_header(ui, "Citizen Profile");

    widgets::info_row(ui, "Username:", &state.user_config.username);
    widgets::info_row(ui, "Password:", "1234 (Ministry-assigned)");

    widgets::section_header(ui, "Regional Settings");

    widgets::info_row(ui, "Time Zone:", &state.user_config.timezone);
    widgets::info_row(ui, "Keyboard:", &state.user_config.keyboard_layout);
    widgets::info_row(ui, "Hostname:", &state.user_config.hostname);

    widgets::section_header(ui, "System Configuration");

    if let Some(disk) = state.get_selected_disk() {
        widgets::info_row(
            ui,
            "Target Disk:",
            &format!("{} ({})", disk.path, disk.size_human()),
        );
    } else if state.preview_mode {
        widgets::info_row(ui, "Target Disk:", "(Preview mode - no disk selected)");
    }

    ui.add_space(20.0);

    if !state.preview_mode {
        widgets::warning_banner(
            ui,
            "Proceeding will ERASE ALL DATA on the selected disk. This action cannot be undone.",
        );
    }

    ui.add_space(10.0);
    ui.label(theme::muted_text(
        "By clicking 'Install', you confirm your eternal loyalty to BigBother.",
    ));
}
