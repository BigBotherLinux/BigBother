use std::sync::{Arc, Mutex};

use crate::state::{CheckStatus, InstallerState, PreflightState};
use crate::{theme, widgets};
use eframe::egui::{self, Color32, RichText, Stroke};

pub fn render(ui: &mut egui::Ui, state: &mut InstallerState) {
    // Start preflight checks on first render
    {
        let mut preflight = state.preflight.lock().unwrap();
        if !preflight.checks_started {
            preflight.checks_started = true;
            drop(preflight);
            run_checks(state.preflight.clone());
        }
    }

    ui.vertical_centered(|ui| {
        ui.add_space(40.0);

        widgets::surveillance_eye(ui, 100.0);

        ui.add_space(20.0);

        ui.label(theme::title_text("Welcome to BigBother"));

        ui.add_space(10.0);

        ui.label(theme::subtitle_text(
            "Our source is open. Your curtains should be too..",
        ));

        ui.add_space(30.0);

        // Preflight check results
        let (uefi_status, uefi_error, internet_status, internet_error) = {
            let preflight = state.preflight.lock().unwrap();
            (
                preflight.uefi,
                preflight.uefi_error.clone(),
                preflight.internet,
                preflight.internet_error.clone(),
            )
        };

        let any_failed =
            uefi_status == CheckStatus::Failed || internet_status == CheckStatus::Failed;
        let any_running = matches!(uefi_status, CheckStatus::Pending | CheckStatus::Running)
            || matches!(internet_status, CheckStatus::Pending | CheckStatus::Running);

        // Dev mode: always show all checks
        // Prod mode: only show if something failed or still running
        let show_checks = !state.production_mode || any_failed || any_running;

        if show_checks {
            ui.label(
                RichText::new("System Clearance")
                    .size(16.0)
                    .strong()
                    .color(theme::TEXT_SECONDARY),
            );
            ui.add_space(10.0);

            if !state.production_mode || uefi_status != CheckStatus::Passed {
                check_row(ui, "UEFI Boot Mode", uefi_status, uefi_error.as_deref());
                ui.add_space(6.0);
            }

            if !state.production_mode || internet_status != CheckStatus::Passed {
                check_row(
                    ui,
                    "Network Connectivity",
                    internet_status,
                    internet_error.as_deref(),
                );
            }

            if any_failed {
                ui.add_space(10.0);
                if theme::secondary_button(ui, "Retry Checks").clicked() {
                    let mut preflight = state.preflight.lock().unwrap();
                    *preflight = PreflightState::default();
                    preflight.checks_started = true;
                    drop(preflight);
                    run_checks(state.preflight.clone());
                }
            }

            // Dev mode warning when checks failed
            if !state.production_mode && any_failed {
                ui.add_space(10.0);
                widgets::warning_banner(ui, "Preview mode: you may proceed despite failed checks.");
            }
        }
    });

    if !state.production_mode {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(10.0);
            if ui.button("Skip Installer (Dev)").clicked() {
                skip_to_defaults(state);
            }
        });
    }

    // Keep repainting while checks are running
    let preflight = state.preflight.lock().unwrap();
    if matches!(preflight.uefi, CheckStatus::Pending | CheckStatus::Running)
        || matches!(
            preflight.internet,
            CheckStatus::Pending | CheckStatus::Running
        )
    {
        ui.ctx().request_repaint();
    }
}

fn check_row(ui: &mut egui::Ui, label: &str, status: CheckStatus, error: Option<&str>) {
    let (icon, icon_color, border_color) = match status {
        CheckStatus::Pending | CheckStatus::Running => {
            ("...", theme::TEXT_MUTED, Color32::from_rgb(50, 50, 70))
        }
        CheckStatus::Passed => ("\u{2713}", theme::ACCENT_GREEN, theme::ACCENT_GREEN),
        CheckStatus::Failed => ("\u{2717}", theme::ACCENT_RED, theme::ACCENT_RED),
    };

    egui::Frame::new()
        .fill(theme::BG_WIDGET)
        .stroke(Stroke::new(1.0, border_color))
        .corner_radius(4)
        .inner_margin(egui::Margin::symmetric(16, 10))
        .show(ui, |ui| {
            ui.set_width(400.0);
            ui.horizontal(|ui| {
                ui.label(RichText::new(icon).size(16.0).strong().color(icon_color));
                ui.add_space(8.0);
                ui.vertical(|ui| {
                    ui.label(RichText::new(label).size(14.0).color(theme::TEXT_PRIMARY));
                    if let Some(err) = error {
                        ui.label(RichText::new(err).size(11.0).color(theme::ACCENT_RED));
                    }
                });
            });
        });
}

fn run_checks(preflight: Arc<Mutex<PreflightState>>) {
    // UEFI check
    {
        let pf = preflight.clone();
        std::thread::spawn(move || {
            {
                pf.lock().unwrap().uefi = CheckStatus::Running;
            }
            let exists = std::path::Path::new("/sys/firmware/efi").exists();
            let mut state = pf.lock().unwrap();
            if exists {
                state.uefi = CheckStatus::Passed;
            } else {
                state.uefi = CheckStatus::Failed;
                state.uefi_error = Some("Not booted in UEFI mode".to_string());
            }
        });
    }

    // Internet connectivity check
    {
        let pf = preflight.clone();
        std::thread::spawn(move || {
            {
                pf.lock().unwrap().internet = CheckStatus::Running;
            }
            let result = std::process::Command::new("ping")
                .args(["-c", "1", "-W", "3", "1.1.1.1"])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();

            let mut state = pf.lock().unwrap();
            match result {
                Ok(exit) if exit.success() => {
                    state.internet = CheckStatus::Passed;
                }
                Ok(_) => {
                    state.internet = CheckStatus::Failed;
                    state.internet_error = Some("No network response".to_string());
                }
                Err(e) => {
                    state.internet = CheckStatus::Failed;
                    state.internet_error = Some(format!("ping: {}", e));
                }
            }
        });
    }
}

/// Sets default values and skips to the summary page for quick testing
fn skip_to_defaults(state: &mut InstallerState) {
    // Accept all disclaimers
    state.disclaimer_format_accepted = true;
    state.disclaimer_unfree_accepted = true;
    state.disclaimer_surveillance_accepted = true;
    state.terms_accepted = true;

    // Set default user config
    state.user_config.username = "test9user".to_string();
    state.user_config.password = "1234".to_string();
    state.user_config.password_confirm = "1234".to_string();
    state.user_config.timezone = "America/New_York".to_string();
    state.user_config.keyboard_layout = "us".to_string();
    state.user_config.hostname = "bigbother-dev".to_string();

    // Accept password theater override
    state.password_theater.accept_ministry_override = true;

    // Features are already enabled by default (FeatureConfig::new())

    // Skip to disk selection page
    state.current_page = crate::state::Page::DiskSelection;
}
