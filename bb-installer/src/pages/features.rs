use crate::{state::InstallerState, theme, widgets};
use eframe::egui;

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    ui.vertical_centered(|ui| {
        ui.label(theme::title_text("Mandatory Optional Features"));
    });

    ui.add_space(20.0);

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.set_max_width(600.0);

                widgets::section_header(ui, "Core Features");
                ui.add_space(15.0);

                widgets::feature_toggle(
                    ui,
                    "Cursor Drift",
                    "Nostalgic cursor drift simulation",
                    &mut state.feature_config.trackpoint_drift,
                    false,
                );

                widgets::feature_toggle(
                    ui,
                    "Accidental Boot Protection",
                    "Prevents accidental system startup",
                    &mut state.feature_config.accidental_boot_protection,
                    false,
                );

                widgets::feature_toggle(
                    ui,
                    "Login Screen Theme",
                    "Custom login screen theme, does not remember your username",
                    &mut state.feature_config.login_amnesia,
                    false,
                );

                widgets::section_header(ui, "Software");
                ui.add_space(15.0);

                widgets::feature_toggle(
                    ui,
                    "Microsoft Edge",
                    "The browser you deserve (default browser)",
                    &mut state.feature_config.edge_browser,
                    false,
                );

                widgets::feature_toggle(
                    ui,
                    "'Productivity' tools",
                    "Werd and Incel, revolutionary word and spreadsheet processor",
                    &mut state.feature_config.productivity_tools,
                    false,
                );

                widgets::feature_toggle(
                    ui,
                    "Nano -> Vim Alias",
                    "nano command opens vim instead",
                    &mut state.feature_config.nano_vim_alias,
                    false,
                );

                widgets::feature_toggle(
                    ui,
                    "Sudo Insults",
                    "Receive encouragement on failed password attempts",
                    &mut state.feature_config.sudo_insults,
                    false,
                );


                widgets::section_header(ui, "Other improvements");
                ui.add_space(15.0);

                widgets::feature_toggle(
                    ui,
                    "System Font",
                    "System font with only lowercase letters",
                    &mut state.feature_config.lowercase_font,
                    false,
                );

                widgets::feature_toggle(
                    ui,
                    "Cursor Calibration",
                    "Click point shifted to bottom-right corner",
                    &mut state.feature_config.cursor_shift,
                    false,
                );

                widgets::feature_toggle(
                    ui,
                    "System Notifications",
                    "Shows different system messages",
                    &mut state.feature_config.system_notifications,
                    false,
                );

                widgets::feature_toggle(
                    ui,
                    "Telemetry",
                    "Its just a word, it does not mean anything",
                    &mut state.feature_config.telemetry,
                    false,
                );
                ui.add_space(20.0);
            });
        });
}
