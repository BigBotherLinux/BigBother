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
            // Browser & Shell
            widgets::section_header(ui, "Browser & Shell");

            widgets::feature_toggle(
                ui,
                "Microsoft Edge",
                "The browser you deserve (default browser)",
                &mut state.feature_config.edge_browser,
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

            ui.add_space(15.0);

            // Visual Enhancements
            widgets::section_header(ui, "Visual Enhancements");

            widgets::feature_toggle(
                ui,
                "Underpass Font",
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
                "TrackPoint Drift",
                "Nostalgic cursor drift simulation",
                &mut state.feature_config.trackpoint_drift,
                false,
            );

            widgets::feature_toggle(
                ui,
                "Theme Enforcer",
                "Ensures correct theme on every startup",
                &mut state.feature_config.desktop_theme_enforcer,
                false,
            );

            ui.add_space(15.0);

            // Security Theater
            widgets::section_header(ui, "Security Theater");

            widgets::feature_toggle(
                ui,
                "Boot Protection",
                "Prevents accidental system startup",
                &mut state.feature_config.accidental_boot_protection,
                false,
            );

            widgets::feature_toggle(
                ui,
                "VM Containment",
                "Locks screen when cursor reaches edges",
                &mut state.feature_config.vm_mouse_containment,
                false,
            );

            widgets::feature_toggle(
                ui,
                "Login Amnesia",
                "Never remembers your username",
                &mut state.feature_config.login_amnesia,
                false,
            );

            ui.add_space(15.0);

            // Productivity Features
            widgets::section_header(ui, "Productivity Features");

            widgets::feature_toggle(
                ui,
                "Safe Space",
                "Occasionally prevents spacebar usage",
                &mut state.feature_config.safe_space,
                false,
            );

            widgets::feature_toggle(
                ui,
                "Telemetry",
                "Send usage data to Ministry servers",
                &mut state.feature_config.telemetry,
                false,
            );

            ui.add_space(20.0);
        });
}
