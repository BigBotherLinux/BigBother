mod complete;
mod disk_selection;
mod features;
mod hostname;
mod installing;
mod keyboard;
mod password_setup;
mod summary;
mod terms;
mod timezone;
mod user_setup;
mod welcome;

use crate::{install::start_installation, state::{InstallerState, InstallStatus, Page}};
use std::sync::Arc;

pub fn render_page(ui: &mut eframe::egui::Ui, state: &mut InstallerState) {
    match state.current_page {
        Page::Welcome => welcome::render(ui),
        Page::TermsOfSubmission => terms::render(ui, state),
        Page::UserSetup => user_setup::render(ui, state),
        Page::PasswordSetup => password_setup::render(ui, state),
        Page::TimezoneSelection => timezone::render(ui, state),
        Page::KeyboardSelection => keyboard::render(ui, state),
        Page::DiskSelection => disk_selection::render(ui, state),
        Page::FeatureSelection => features::render(ui, state),
        Page::HostnameSetup => hostname::render(ui, state),
        Page::Summary => summary::render(ui, state),
        Page::Installing => installing::render(ui, state),
        Page::Complete => complete::render(ui, state),
    }
}

pub fn start_install_if_ready(state: &mut InstallerState) {
    if state.preview_mode {
        let progress = Arc::clone(&state.install_progress);
        std::thread::spawn(move || {
            simulate_installation(progress);
        });
    } else {
        start_installation(state);
    }
}

fn simulate_installation(progress: Arc<std::sync::Mutex<crate::state::InstallProgress>>) {
    let steps = [
        (InstallStatus::Partitioning, "Simulating partition creation..."),
        (InstallStatus::Formatting, "Simulating filesystem formatting..."),
        (InstallStatus::Mounting, "Simulating mount operations..."),
        (InstallStatus::CopyingFlake, "Simulating flake deployment..."),
        (InstallStatus::GeneratingConfig, "Simulating config generation..."),
        (InstallStatus::RunningNixosInstall, "Simulating nixos-install (this would take a while in real life)..."),
        (InstallStatus::Finalizing, "Simulating finalization..."),
        (InstallStatus::Complete, "Simulation complete!"),
    ];

    for (status, message) in steps {
        if let Ok(mut p) = progress.lock() {
            p.status = status;
            p.output_log.push(format!("[PREVIEW] {}", message));
        }
        std::thread::sleep(std::time::Duration::from_millis(800));
    }
}
