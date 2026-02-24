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

                let telemetry_before = state.feature_config.telemetry;
                widgets::feature_toggle(
                    ui,
                    "Telemetry",
                    "Its just a word, it does not mean anything",
                    &mut state.feature_config.telemetry,
                    false,
                );
                // If telemetry was just disabled, start the cascade
                if telemetry_before && !state.feature_config.telemetry {
                    state.feature_cascade_active = true;
                    state.feature_cascade_start_time = Some(ui.ctx().input(|i| i.time));
                    state.feature_cascade_index = 0;
                }

                // Handle the cascade animation
                if state.feature_cascade_active {
                    if let Some(start_time) = state.feature_cascade_start_time {
                        let elapsed = ui.ctx().input(|i| i.time) - start_time;
                        let delay_per_feature = 0.5; // between each feature disabling
                        let features_to_disable = (elapsed / delay_per_feature).floor() as usize;

                        if features_to_disable > state.feature_cascade_index {
                            state.feature_cascade_index = features_to_disable;
                        }

                        // Disable features one by one based on cascade index
                        match state.feature_cascade_index {
                            10 => {}
                            9 => state.feature_config.trackpoint_drift = false,
                            8 => state.feature_config.accidental_boot_protection = false,
                            7 => state.feature_config.login_amnesia = false,
                            6 => state.feature_config.edge_browser = false,
                            5 => state.feature_config.productivity_tools = false,
                            4 => state.feature_config.nano_vim_alias = false,
                            3 => state.feature_config.sudo_insults = false,
                            2 => state.feature_config.lowercase_font = false,
                            1 => state.feature_config.cursor_shift = false,
                            0 => state.feature_config.system_notifications = false,
                            _ => {
                                // All done - re-enable telemetry
                                state.feature_config.telemetry = true;
                                state.feature_cascade_active = false;
                                state.feature_cascade_start_time = None;
                                state.feature_cascade_index = 0;
                            }
                        }

                        // Request repaint to continue animation
                        ui.ctx().request_repaint();
                    }
                }
                ui.add_space(20.0);
            });
        });
}
