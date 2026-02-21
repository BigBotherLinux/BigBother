use crate::{install, state::InstallerState, theme, widgets};
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.label(theme::title_text("Storage Requisition"));
        ui.add_space(5.0);
        ui.label(theme::muted_text(
            "Select a device for BigBother deployment",
        ));
        ui.add_space(5.0);
        ui.label(theme::muted_text(
            "We didnt really bother implementing partitioning or encryption, so you just have to format the entire thing.",
        ));
    });

    ui.add_space(20.0);

    if !state.production_mode {
        widgets::preview_mode_banner(ui);
        ui.add_space(10.0);
    }

    ui.horizontal(|ui| {
        if ui.button("Scan for devices").clicked() {
            state.available_disks = if state.production_mode {
                install::detect_disks()
            } else {
                install::mock_disks()
            };
        }

        ui.label(theme::muted_text(&format!(
            "{} device(s) detected",
            state.available_disks.len()
        )));
    });

    ui.add_space(15.0);

    widgets::section_header(ui, "Available Storage Devices");

    if state.available_disks.is_empty() {
        ui.add_space(20.0);
        ui.label(theme::muted_text(
            "No suitable devices found. Click 'Scan for devices' to refresh.",
        ));
    } else {
        egui::ScrollArea::vertical()
            .max_height(250.0)
            .show(ui, |ui| {
                for (idx, disk) in state.available_disks.iter().enumerate() {
                    let is_selected = state.selected_disk == Some(idx);

                    let response = widgets::disk_card(
                        ui,
                        &disk.name,
                        &disk.size_human(),
                        &disk.model,
                        is_selected,
                    );

                    if response.clicked() {
                        state.selected_disk = Some(idx);
                    }

                    ui.add_space(5.0);
                }
            });
    }

    ui.add_space(15.0);

    if let Some(disk) = state.get_selected_disk() {
        widgets::warning_banner(
            ui,
            &format!(
                "WARNING: All data on {} ({}) will be permanently deleted. This cannot be undone.",
                disk.path,
                disk.size_human()
            ),
        );
    }
}
